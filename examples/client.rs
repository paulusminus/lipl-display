use lipl_gatt_zbus::listen_background;
use std::sync::mpsc::channel;
use simple_logger::SimpleLogger;
use tokio::main;

#[main(flavor = "current_thread")]
async fn main() {
    SimpleLogger::new().init().unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let (values_tx, values_rx) = channel();
    listen_background(move |message| {
        values_tx
            .send(message)
            .map_err(lipl_display_common::Error::Send)
            // .map_err(lipl_gatt_zbus::Error::Common)
    });

    while let Ok(message) = values_rx.recv() {
        log::info!("{:?}", message);
    }
}
