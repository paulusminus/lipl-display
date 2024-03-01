use std::{fs::OpenOptions, io::Write, ops::Not};

use futures_util::{future::ready, Future, FutureExt, StreamExt};
use lipl_display_common::{HandleMessage, LiplScreen, Message};
use lipl_gatt_bluer::listen_stream;

struct Out<W>
where
    W: Write,
{
    out: W,
}

impl<W> Out<W>
where
    W: Write,
{
    pub fn new(w: W) -> Self
    where
        W: Write,
    {
        Self { out: w }
    }

    pub fn send_json(&mut self, screen: &LiplScreen) {
        self.out.write_all(serde_json::to_string(screen).unwrap().as_bytes()).unwrap();
        self.out.write("\n".as_bytes()).unwrap();
    }
}

fn not_is_stop(message: &Message) -> impl Future<Output = bool> {
    is_stop(message).map(Not::not)
}

fn is_stop(message: &Message) -> impl Future<Output = bool> {
    ready(message.is_stop())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let screen = LiplScreen::new(false, "Wachten op connectie", 30.0);
    // let mut out = Out::new(std::io::stdout());
    let mut fifo = OpenOptions::new().write(true).open("lipl-out").map(Out::new).unwrap();

    match listen_stream().await {
        Ok(stream) => {
            stream
                .take_while(not_is_stop)
                .scan(screen, |screen, message| {
                    screen.handle_message(message);
                    ready(Some(screen.clone()))
                })
                .for_each(|screen| ready(fifo.send_json(&screen)))
                .await;
        }
        Err(error) => {
            eprintln!("{}", error);
        }
    };
}
