use std::sync::mpsc::TrySendError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Send error: {0}")]
    Send(#[from] TrySendError<()>),

    #[error("No bluetooth adapter found")]
    NoBluetooth,

    #[error("Cancelled")]
    Cancelled,

    #[error("Cannot send poweroff to login")]
    Poweroff,

    #[error("Hostname environment variable no set")]
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
