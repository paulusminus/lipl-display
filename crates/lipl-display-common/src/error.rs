use std::sync::mpsc::{SendError, TrySendError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Try Send error: {0}")]
    TrySend(#[from] TrySendError<()>),

    #[error("Send error: {0}")]
    Send(#[from] SendError<crate::Message>),

    #[error("No bluetooth adapter found")]
    NoBluetooth,

    #[error("Cancelled")]
    Cancelled,

    #[error("Cannot send poweroff to login")]
    Poweroff,

    #[error("Hostname environment variable not set")]
    Hostname,

    #[error("Failed to call callback")]
    Callback,

    #[error("Error creating tokio runtime")]
    Runtime,

    #[error("Json serialization")]
    JsonSerialization,

    #[error("Parsing Gatt Characteristic value failed")]
    GattCharaceristicValueParsingFailed(String),
}
