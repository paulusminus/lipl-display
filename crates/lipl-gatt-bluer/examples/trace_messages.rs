use lipl_display_common::{BackgroundThread, Message};
use log::{error, info};
use std::sync::mpsc::{Sender, channel};
use tokio::main;

fn create_callback(tx: Sender<Message>) -> impl Fn(Message) {
    move |message| {
        if let Err(error) = tx.send(message) {
            error!("Error sending: {}", error);
        }
    }
}

#[main(flavor = "current_thread")]
async fn main() {
    // let filter = std::env::var("RUST_LOG").unwrap_or("info".to_owned());
    // tracing_subscriber::fmt().with_env_filter(filter).init();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let (values_tx, values_rx) = channel();
    let mut gatt = lipl_gatt_bluer::ListenBluer::new(create_callback(values_tx));

    while let Ok(message) = values_rx.recv() {
        info!("{:?}", message);
    }

    gatt.stop();
}
