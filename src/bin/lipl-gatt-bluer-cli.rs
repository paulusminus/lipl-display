use futures::StreamExt;
use lipl_gatt_bluer::message::{Command, Message};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx_values, mut rx_values) = lipl_gatt_bluer::create_channel();
    let (mut tx_cancel, rx_cancel) = lipl_gatt_bluer::create_cancel();

    let task1 = async move {
        lipl_gatt_bluer::listen(rx_cancel, tx_values).await?;
        Ok::<(), Box<dyn std::error::Error>>(())
    };

    let task2 = async move {
        while let Some(message) = rx_values.next().await {
            match message {
                Message::Part(part) => { println!("Received part: {}", part); },
                Message::Status(status) => { println!("Received status: {}", status); },
                Message::Command(command) => {
                    if command == Command::Poweroff {
                        match tx_cancel.try_send(()) {
                            Ok(_) => { println!("Cancel send to listen task"); },
                            Err(_) => { eprintln!("Failed to send cancel to listen task"); }
                        };
                    }
                    else {
                        println!("Received command: {:?}", command);
                    }
                },
            }
        }
        Ok::<(), Box<dyn std::error::Error>>(())
    };

    tokio::try_join!(task1, task2)?;
    Ok(())
}