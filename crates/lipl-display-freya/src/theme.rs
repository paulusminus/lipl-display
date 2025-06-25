#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Theme {
    Dark,
    Light,
}

impl From<bool> for Theme {
    fn from(dark: bool) -> Self {
        if dark { Theme::Dark } else { Theme::Light }
    }
}

impl Theme {
    pub fn dark() -> Self {
        Theme::Dark
    }
    pub fn light() -> Self {
        Theme::Light
    }
    pub fn bg_color(&self) -> &'static str {
        match self {
            Self::Dark => "black",
            Self::Light => "white",
        }
    }
    pub fn fg_color(&self) -> &'static str {
        match self {
            Self::Dark => "white",
            Self::Light => "black",
        }
    }
}
