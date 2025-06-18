use futures_util::{TryStream, TryStreamExt, future::ready};
use serde::de::DeserializeOwned;
use std::io::Error;
use std::path::Path;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;

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
