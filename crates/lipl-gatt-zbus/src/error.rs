pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Error {
    source: Box<dyn std::error::Error + Send + Sync>,
}

impl<E: std::error::Error + Send + Sync + 'static> From<E> for Error {
    fn from(source: E) -> Self {
        Self {
            source: Box::new(source),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Other error: {}", self.source)
    }
}

pub trait ErrInto<T> {
    fn err_into(self) -> Result<T>;
}

impl<T, E: Into<Error>> ErrInto<T> for Result<T, E> {
    fn err_into(self) -> Result<T, Error> {
        self.map_err(Into::into)
    }
}
