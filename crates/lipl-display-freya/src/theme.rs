use freya::prelude::Color;

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
    pub fn set(&mut self, theme: Theme) {
        *self = theme;
    }

    pub fn dark() -> Self {
        Theme::Dark
    }
    pub fn light() -> Self {
        Theme::Light
    }
    pub fn bg_color(&self) -> Color {
        match self {
            Self::Dark => Color::BLACK,
            Self::Light => Color::WHITE,
        }
    }
    pub fn fg_color(&self) -> Color {
        match self {
            Self::Dark => Color::WHITE,
            Self::Light => Color::BLACK,
        }
    }
}
