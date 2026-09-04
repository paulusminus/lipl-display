use lipl_display_common::Message;
use thiserror::Error;
use tokio::sync::mpsc::error::TrySendError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Bluer error: {0}")]
    Bluer(#[from] bluer::Error),

    #[error("Common error: {0}")]
    Common(#[from] lipl_display_common::Error),

    #[error("Callback")]
    Callback,

    #[error("Send: {0}")]
    Send(#[from] TrySendError<Message>),
}
