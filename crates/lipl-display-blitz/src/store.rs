use dioxus::prelude::*;

use crate::args::Args;

#[derive(Store)]
pub struct Lipl {
    pub font_size: u32,
    pub dark: bool,
    pub part: String,
    pub status: String,
    pub wait_message: String,
    pub timeout: u64,
}

// impl Default for Lipl {
//     fn default() -> Self {
//         Self {
//             dark: DEFAULT_DARK,
//             font_size: DEFAULT_FONT_SIZE,
//             part: DEFAULT_PART.to_owned(),
//             status: DEFAULT_STATUS.to_owned(),
//             wait_message: DEFAULT_STATUS.to_owned(),
//             timeout: DEFAULT_TIMEOUT,
//         }
//     }
// }

impl From<Args> for Lipl {
    fn from(args: Args) -> Self {
        Self {
            dark: args.light,
            font_size: args.font_size,
            part: String::new(),
            status: args.wait_message.clone(),
            wait_message: args.wait_message,
            timeout: args.timeout,
        }
    }
}
