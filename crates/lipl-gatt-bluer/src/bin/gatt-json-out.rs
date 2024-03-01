use std::ops::Not;

use futures::{future::ready, Future, FutureExt, StreamExt};
use lipl_display_common::Message;
use lipl_gatt_bluer::listen_stream;

fn not_is_stop(message: &Message) -> impl Future<Output = bool> {
    is_stop(message).map(Not::not)
}

fn is_stop(message: &Message) -> impl Future<Output = bool> {
    ready(message.is_stop())
}

async fn to_json_stdout(message: Message) {
    let now = std::time::Instant::now();
    println!("{}", serde_json::to_string(&message).unwrap());
    log::info!(
        "Outputting json message took {} microseconds",
        now.elapsed().as_micros()
    );
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    match listen_stream().await {
        Ok(stream) => {
            stream
                .take_while(not_is_stop)
                .for_each(to_json_stdout)
                .await;
        }
        Err(error) => {
            eprintln!("{}", error);
        }
    };
}
