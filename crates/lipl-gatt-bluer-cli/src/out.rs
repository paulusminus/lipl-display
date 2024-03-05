use std::io::{Stdout, Write};
use serde::Serialize;
use crate::error::{Error, ErrInto};

pub struct Out<W = Stdout>
where
    W: Write,
{
    out: W,
}

impl Default for Out {
    fn default() -> Self {
        Self {
            out: std::io::stdout(),
        }
    }
}

impl<W> Out<W>
where
    W: Write,
{
    pub fn send_json<S: Serialize>(&mut self, serializable: &S) -> Result<(), Error> {
        let json = serde_json::to_string(serializable).err_into()? + "\n";
        self.out.write_all(json.as_bytes()).err_into()?;
        self.out.flush().err_into()
    }
}
