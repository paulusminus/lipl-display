use futures::StreamExt;
use lipl_gatt_zbus::GattListener;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt::init();
    GattListener::default().map(|message| println!("{:?}", message)).collect::<Vec<_>>().await;
}
