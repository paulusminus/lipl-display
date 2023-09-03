use lipl_display_common::Listen;
use lipl_gatt_zbus::ListenZbus;
use std::sync::mpsc::channel;

fn main() {
    let (values_tx, values_rx) = channel();
    let mut gatt = ListenZbus {};
    gatt.listen_background(move |message| {
        if let Err(error) = values_tx.send(message) {
            tracing::error!("Error sending message: {}", error);
        }
    });

    while let Ok(message) = values_rx.recv() {
        println!("{:?}", message);
    }
}
