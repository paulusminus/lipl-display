use futures::StreamExt;
use lipl_gatt_zbus::GattListener;
use tokio::{
    select,
    time::{Duration, sleep},
};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut listener = GattListener::default();

    loop {
        select! {
            message = listener.next() => {
                match message {
                    Some(message) => {
                        println!("{:?}", message);
                    }
                    None => {
                        break;
                    }
                }
            }
            _ = sleep(Duration::from_secs(300)) => {
                break;
            }
        }
    }

    if let Err(err) = listener.await {
        eprintln!("Error: {}", err);
    }

    println!("Finished");
}
