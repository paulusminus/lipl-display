use std::io::Read;

use clap::Parser;
use error::{ErrInto, Error};
use futures_util::StreamExt;
use lipl_display_common::{Command, HandleMessage, LiplScreen, Message};
use lipl_gatt_bluer::listen_stream;
use login_poweroff_reboot::{shutdown, Shutdown};
use param::Params;

mod error;
mod out;
mod param;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let params = Params::parse();
    let mut screen = LiplScreen::new(params.dark, params.font_size.into());
    let mut out = out::Out::default();

    let mut stream = listen_stream().await?.boxed();
    out.send_json(&screen)?;
    while let Some(message) = stream.next().await {
        if message == Message::Command(Command::Poweroff) {
            shutdown(Shutdown::Poweroff)(1000).err_into()?;
            break;
        } else if message == Message::Command(Command::Exit) {
            break;
        } else {
            screen.handle_message(message);
            out.send_json(&screen)?;
        }
    }

    std::io::stdin().take(1);
    Ok(())
}
