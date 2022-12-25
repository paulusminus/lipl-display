use thiserror::Error;
pub use lipl_display_common::Error as CommonError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Bluer error: {0}")]
    Bluer(#[from] bluer::Error),

    #[error("Common error: {0}")]
    Common(#[from] CommonError),

    #[error("Callback")]
    Callback,
}
