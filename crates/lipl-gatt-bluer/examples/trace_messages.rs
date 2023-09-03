use std::sync::mpsc::channel;
use tokio::main;
use lipl_display_common::Listen;

#[main(flavor = "current_thread")]
async fn main() {
    let filter = std::env::var("RUST_LOG").unwrap_or("info".to_owned());
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let (values_tx, values_rx) = channel();
    let mut gatt = lipl_gatt_bluer::ListenBluer { sender: None };
    gatt.listen_background(move |message| {
        if let Err(error) = values_tx.send(message) {
            tracing::error!("Error sending: {}", error);
        }
    });

    while let Ok(message) = values_rx.recv() {
        tracing::info!("{:?}", message);
    }
}
