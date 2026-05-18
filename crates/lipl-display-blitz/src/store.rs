use dioxus::prelude::*;

const DEFAULT_STATUS: &str = "Even geduld a.u.b. ...";
const DEFAULT_PART: &str = "";
const DEFAULT_DARK: bool = false;
const DEFAULT_FONT_SIZE: u32 = 30;

#[derive(Store)]
pub struct Lipl {
    font_size: u32,
    dark: bool,
    part: String,
    status: String,
}

impl Default for Lipl {
    fn default() -> Self {
        Self {
            dark: DEFAULT_DARK,
            font_size: DEFAULT_FONT_SIZE,
            part: DEFAULT_PART.to_owned(),
            status: DEFAULT_STATUS.to_owned(),
        }
    }
}
#[store(pub)]
impl<Lens> Store<Lipl, Lens> {
    fn get_part_lines(&self) -> Vec<String> {
        self.part()
            .cloned()
            .split('\n')
            .map(|s| s.trim().to_owned())
            .collect()
    }
}
