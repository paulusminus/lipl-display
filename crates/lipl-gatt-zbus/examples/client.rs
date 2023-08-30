use lipl_display_common::Listen;
use lipl_gatt_zbus::ListenZbus;
use std::sync::mpsc::channel;

fn main() {
    let (values_tx, values_rx) = channel();
    let gatt = ListenZbus::new();
    gatt.listen_background(move |message| {
        values_tx
            .send(message)
            .map_err(lipl_display_common::Error::Send)
            // .map_err(lipl_gatt_zbus::Error::Common)
    });

    while let Ok(message) = values_rx.recv() {
        println!("{:?}", message);
    }
}
