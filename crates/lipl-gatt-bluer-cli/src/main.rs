use std::{io::Write, ops::Not};

use futures_util::{future::ready, Future, FutureExt, StreamExt, TryStreamExt};
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

    pub fn send_json(&mut self, s: String) -> Result<(), Error> {
        self.out.write_all(s.as_bytes()).map_err(Error::from)?;
        self.out.write_all("\n".as_bytes()).map_err(Error::from)?;
        self.out.flush().map_err(Error::from)
    }
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Json: {0}")]
    Json(#[from] serde_json::Error),
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
    let mut out = Out::new(std::io::stdout());
    // let mut fifo = OpenOptions::new().write(true).open("lipl-out").map(Out::new).unwrap();

    match listen_stream().await {
        Ok(mut stream) => {
            match stream
                .by_ref()
                .take_while(not_is_stop)
                .scan(screen, |screen, message| {
                    screen.handle_message(message);
                    ready(Some(screen.clone()))
                })
                .map(|screen| serde_json::to_string(&screen).map_err(Error::from))
                .try_for_each(|s| {
                    let r = out.send_json(s);
                    ready(r)
                })
                .await
            {
                Ok(_) => {
                    if let Some(message) = stream.next().await {
                        if message == Message::Command(lipl_display_common::Command::Poweroff) {
                            if let Err(error) = login_poweroff_reboot::reboot(1000) {
                                eprintln!("Error: {}", error);
                            }
                        }
                    }
                }
                Err(error) => eprintln!("Error: {}", error),
            }
        }
        Err(error) => {
            eprintln!("{}", error);
        }
    };
}
