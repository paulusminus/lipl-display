use thiserror::Error;
use futures::channel::mpsc::TrySendError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Bluer error: {0}")]
    Bluer(#[from] bluer::Error),

    #[error("Send error: {0}")]
    Send(#[from] TrySendError<()>),

    #[error("No bluetooth adapter found")]
    NoBluetooth,

    #[error("Cancelled")]
    Cancelled,

    #[error("Cannot send poweroff to login")]
    Poweroff,

    #[error("Hostname environment variable no set!")]
    Hostname,
}
