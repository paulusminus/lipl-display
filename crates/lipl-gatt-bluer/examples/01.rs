use std::sync::mpsc::channel;
use tokio::main;

#[main(flavor = "current_thread")]
async fn main() {
    let filter = std::env::var("RUST_LOG").unwrap_or("info".to_owned());
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let (values_tx, values_rx) = channel();
    lipl_gatt_bluer::listen_background(move |message| {
        values_tx
            .send(message)
            .map_err(lipl_display_common::Error::Send)
            .map_err(lipl_gatt_bluer::Error::Common)
    });

    while let Ok(message) = values_rx.recv() {
        tracing::info!("{:?}", message);
    }
}
