use std::sync::mpsc::channel;
use lipl_gatt_bluer::message::{Message};
use lipl_gatt_bluer::Error;
use simple_logger::SimpleLogger;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    SimpleLogger::new().init().unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let (values_tx, values_rx) = channel::<Message>();
    lipl_gatt_bluer::listen_background(move |message| {
        values_tx.send(message).map_err(|_| Error::Callback)
    });

    while let Ok(message) = values_rx.recv() {
        log::info!("{:?}", message);
    }
}
