use std::sync::mpsc::channel;
use simple_logger::SimpleLogger;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    SimpleLogger::new().init().unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let (values_tx, values_rx) = channel();
    lipl_gatt_bluer::listen_background(move |message| {
        values_tx
            .send(message)
            .map_err(lipl_display_common::Error::from)
            .map_err(lipl_gatt_bluer::Error::from)
    });

    while let Ok(message) = values_rx.recv() {
        log::info!("{:?}", message);
    }
}
