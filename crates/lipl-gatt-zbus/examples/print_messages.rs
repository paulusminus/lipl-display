use futures::StreamExt;
use lipl_gatt_zbus::GattListener;
use tokio::{
    select,
    time::{Duration, sleep},
};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    tracing_subscriber::fmt::init();

    let listener = GattListener::default();
    select! {
        _ = listener.map(|message| println!("{:?}", message)).collect::<Vec<_>>() => {}
        _ = sleep(Duration::from_secs(1)) => {}
    }
    println!("Finished");
}
