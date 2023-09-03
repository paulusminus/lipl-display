use std::sync::mpsc::{channel, Sender};
use tokio::main;
use lipl_display_common::{Listen, Message};

fn create_callback(tx: Sender<Message>) -> impl Fn(Message) {
    move |message| {
        if let Err(error) = tx.send(message) {
            tracing::error!("Error sending: {}", error);
        }
    }
}

#[main(flavor = "current_thread")]
async fn main() {
    let filter = std::env::var("RUST_LOG").unwrap_or("info".to_owned());
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let (values_tx, values_rx) = channel();
    let mut gatt = lipl_gatt_bluer::ListenBluer::new(create_callback(values_tx));

    while let Ok(message) = values_rx.recv() {
        tracing::info!("{:?}", message);
    }

    gatt.stop();
}
