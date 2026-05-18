use clap::Parser;

use crate::constant::{DEFAULT_FONT_SIZE, DEFAULT_STATUS, DEFAULT_TIMEOUT};

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = DEFAULT_FONT_SIZE)]
    pub font_size: u32,

    #[arg(short, long, default_value_t = DEFAULT_STATUS.to_owned())]
    pub wait_message: String,

    #[arg(short, long)]
    pub light: bool,

    #[arg(short, long, default_value_t = DEFAULT_TIMEOUT)]
    pub timeout: u64,
}
