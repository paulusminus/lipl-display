use std::sync::mpsc::channel;
use lipl_gatt_bluer::message::{Message};
use lipl_gatt_bluer::Error;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (values_tx, values_rx) = channel::<Message>();
    lipl_gatt_bluer::listen_background(move |message| {
        values_tx.send(message).map_err(|_| Error::Callback)
    });

    while let Ok(message) = values_rx.recv() {
        println!("{:?}", message);
    }
}
