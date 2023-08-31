use std::sync::mpsc::channel;
use tokio::main;
use lipl_display_common::Listen;

#[main(flavor = "current_thread")]
async fn main() {
    let filter = std::env::var("RUST_LOG").unwrap_or("info".to_owned());
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let (values_tx, values_rx) = channel();
    let gatt = lipl_gatt_bluer::ListenBluer {};
    gatt.listen_background(move |message| {
        values_tx
            .send(message)
            .map_err(lipl_display_common::Error::Send)
    });

    while let Ok(message) = values_rx.recv() {
        tracing::info!("{:?}", message);
    }
}
