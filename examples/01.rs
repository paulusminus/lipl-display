use lipl_gatt_bluer::{Result};
use lipl_gatt_bluer::message::{Command, Message};
use futures::StreamExt;


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut s = lipl_gatt_bluer::listen_stream().await?;

    while let Some(value) = s.next().await {
        if value == Message::Command(Command::Poweroff) || value == Message::Command(Command::Exit) { break; }
        println!("{:?}", value);
    }
    
    Ok(())
}
