use thiserror::Error;
pub use lipl_display_common::Error as CommonError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("ZBus error: {0}")]
    ZBus(#[from] zbus::Error),

    #[error("Common error: {0}")]
    Common(#[from] CommonError),

    #[error("Callback")]
    Callback,
}