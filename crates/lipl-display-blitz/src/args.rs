use clap::Parser;

#[derive(Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub initial_font_size: Option<u32>,

    #[arg(short, long)]
    pub wait_message: Option<String>,

    #[arg(short, long)]
    pub light: bool,

    #[arg(short, long)]
    pub timeout: Option<u64>,
}
