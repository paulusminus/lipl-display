use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Part(String);

impl From<String> for Part {
    fn from(part: String) -> Self {
        Self(part)
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
