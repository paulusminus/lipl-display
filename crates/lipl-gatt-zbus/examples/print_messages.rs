use std::{io::Result as IOResult, pin::pin};

use futures::{
    TryStreamExt,
    future::{Either, select},
    stream::TryNext,
};
use lipl_display_common::Message;
use lipl_gatt_zbus::GattListener;
use tokio::time::{Duration, Sleep, sleep};
use tracing::{error, info};

enum Choice {
    Next(Option<Message>),
    Sleep,
}

async fn choice(next: TryNext<'_, GattListener>, sleep: Sleep) -> IOResult<Choice> {
    match select(pin!(next), pin!(sleep)).await {
        Either::Left((next, _)) => next
            .map(Choice::Next)
            .inspect_err(|error| error!("{}", error)),
        Either::Right((_, _)) => Ok(Choice::Sleep),
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> IOResult<()> {
    tracing_subscriber::fmt::init();

    let mut listener = GattListener::default();

    while let Choice::Next(Some(message)) =
        choice(listener.try_next(), sleep(Duration::from_secs(3000))).await?
    {
        info!("{:?}", message)
    }

    if let Err(err) = listener.await {
        error!("Error: {}", err);
    }

    info!("Finished");
    Ok(())
}
