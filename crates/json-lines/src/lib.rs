use futures_util::{TryStream, TryStreamExt, future::ready};
use serde::de::DeserializeOwned;
use std::io::Error;
use std::path::Path;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;

pub struct Lines<R: AsyncBufRead> {
    reader: LinesStream<R>,
}

impl<R: AsyncBufRead> From<R> for Lines<R> {
    fn from(reader: R) -> Self {
        Self {
            reader: LinesStream::new(reader.lines()),
        }
    }
}

impl<R: AsyncBufRead> Lines<R> {
    pub fn json_lines<O>(self) -> impl TryStream<Ok = O, Error = Error>
    where
        O: DeserializeOwned,
    {
        self.reader
            .and_then(|line| ready(serde_json::from_str::<O>(&line).map_err(Error::other)))
    }
}

pub async fn cursor_reader(s: &'static str) -> Result<impl AsyncBufRead, Error> {
    Ok(s.as_bytes())
}

pub async fn file_reader<P>(path: P) -> Result<impl AsyncBufRead, Error>
where
    P: AsRef<Path>,
{
    tokio::fs::File::open(path)
        .await
        .map_err(Error::other)
        .map(BufReader::new)
}

pub fn lines<O>(r: impl AsyncBufRead) -> impl TryStream<Ok = O, Error = Error>
where
    O: DeserializeOwned,
{
    LinesStream::new(r.lines())
        .and_then(|line| ready(serde_json::from_str::<O>(&line).map_err(Error::other)))
}

#[cfg(test)]
mod test {
    use futures_util::TryStreamExt;
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct Person {
        name: String,
        age: u32,
    }

    #[tokio::test]
    async fn test_json_lines() {
        let json = r#"{"name": "Paul Min", "age": 30}
            {"name": "John Doe", "age": 25}"#;
        let lines = Lines::from(json.as_bytes());
        let result: Vec<Person> = lines.json_lines::<Person>().try_collect().await.unwrap();
        assert_eq!(
            result,
            vec![
                Person {
                    name: "Paul Min".to_string(),
                    age: 30
                },
                Person {
                    name: "John Doe".to_string(),
                    age: 25
                }
            ]
        );
    }
}
