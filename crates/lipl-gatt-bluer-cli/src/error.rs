#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Gatt: {0}")]
    Gatt(#[from] lipl_gatt_bluer::Error),

    #[error("Login poweroff reboot: {0}")]
    Login(#[from] login_poweroff_reboot::Error),
}

pub trait ErrInto<T> {
    fn err_into(self) -> Result<T, Error>;
}

impl<T, E> ErrInto<T> for Result<T, E>
where
    E: Into<Error>,
{
    fn err_into(self) -> Result<T, Error> {
        self.map_err(Into::into)
    }
}
