use freya::prelude::*;

#[derive(Props, Debug, Clone, PartialEq)]
pub struct Theme {
    dark: bool,
}

impl Theme {
    pub fn dark() -> Self {
        Theme { dark: true }
    }
    pub fn light() -> Self {
        Theme { dark: false }
    }
    pub fn bg_color(&self) -> &'static str {
        match self.dark {
            true => "black",
            false => "light",
        }
    }
    pub fn fg_color(&self) -> &'static str {
        match self.dark {
            true => "white",
            false => "black",
        }
    }
}
