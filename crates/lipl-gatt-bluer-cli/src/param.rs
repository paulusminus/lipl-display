use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Params {
    #[arg(short, long, required = true, action = clap::ArgAction::Set)]
    pub dark: bool,
    #[arg(short, long, required = true)]
    pub font_size: u16,
}
