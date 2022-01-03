use futures::StreamExt;
use lipl_gatt_bluer::message::{Command, Message};
use lipl_gatt_bluer::{Error, Result};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let (tx_values, mut rx_values) = lipl_gatt_bluer::create_channel();
    let (mut tx_cancel, rx_cancel) = lipl_gatt_bluer::create_cancel();

    let task1 = async move {
        lipl_gatt_bluer::listen(rx_cancel, tx_values).await
    };

    let task2 = async move {

        while let Some(message) = rx_values.next().await {
            match message {
                Message::Part(part) => { println!("Received part: {}", part); },
                Message::Status(status) => { println!("Received status: {}", status); },
                Message::Command(command) => {
                    if command == Command::Poweroff {
                        tx_cancel.try_send(())?;
                        return Err(Error::Cancelled);
                    }
                    else {
                        println!("Received command: {:?}", command);
                    }
                },
            }
        }
        Ok::<(), Error>(())
    };

    if let Err(Error::Cancelled) = tokio::try_join!(task1, task2) {
        login_poweroff_reboot::poweroff().map_err(|_| Error::Poweroff)?;
    };
    Ok(())
}