use dioxus::prelude::*;

use crate::{
    args::Args,
    constant::{DEFAULT_FONT_SIZE, DEFAULT_PART, DEFAULT_STATUS, DEFAULT_TIMEOUT},
};

#[derive(Store)]
pub struct Lipl {
    font_size: u32,
    dark: bool,
    part: String,
    status: String,
    wait_message: String,
    timeout: u64,
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
            font_size: args.initial_font_size.unwrap_or(DEFAULT_FONT_SIZE),
            part: DEFAULT_PART.to_owned(),
            status: args
                .wait_message
                .clone()
                .unwrap_or(DEFAULT_STATUS.to_string()),
            wait_message: args
                .wait_message
                .clone()
                .unwrap_or(DEFAULT_STATUS.to_string()),
            timeout: args.timeout.unwrap_or(DEFAULT_TIMEOUT),
        }
    }
}
