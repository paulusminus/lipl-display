use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Status(String);

impl Status {
    pub fn set_text(&mut self, text: String) {
        self.0 = text;
    }
}

impl From<String> for Status {
    fn from(status: String) -> Self {
        Status(status)
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
