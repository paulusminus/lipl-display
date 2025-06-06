use lipl_display_common::BackgroundThread;
use lipl_gatt_zbus::ListenZbus;
use std::sync::mpsc::channel;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt::init();

    let (values_tx, values_rx) = channel();
    let mut gatt = ListenZbus {};
    gatt.listen_background(move |message| {
        if let Err(error) = values_tx.send(message) {
            tracing::error!("Error sending message: {}", error);
        }
    }).await.unwrap();

    while let Ok(message) = values_rx.recv() {
        println!("{:?}", message);
    }

    gatt.stop();
}
