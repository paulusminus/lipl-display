use std::{fmt::Display, io::Result as IOResult, pin::pin};

use futures::{FutureExt, TryFutureExt, TryStreamExt, future::select, stream::TryNext};
use lipl_display_common::Message;
use lipl_gatt_zbus::GattListener;
use tokio::time::{Duration, Sleep, sleep};
use tracing::{error, info};

trait TraceExt<T, E> {
    fn trace_error(self) -> Result<T, E>;
}

impl<T, E: Display> TraceExt<T, E> for Result<T, E> {
    fn trace_error(self) -> Result<T, E> {
        self.inspect_err(|e| error!("{}", e))
    }
}

enum Choice {
    Next(Option<Message>),
    Sleep,
}

impl From<Option<Message>> for Choice {
    fn from(value: Option<Message>) -> Self {
        Self::Next(value)
    }
}

impl From<()> for Choice {
    fn from(_: ()) -> Self {
        Self::Sleep
    }
}

async fn choice(try_next: TryNext<'_, GattListener>, sleep: Sleep) -> IOResult<Choice> {
    select(
        pin!(try_next.map_ok(Choice::from)),
        pin!(sleep.map(IOResult::Ok).map_ok(Choice::from)),
    )
    .await
    .factor_first()
    .0
    .trace_error()
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mut listener = GattListener::default();

    while let Choice::Next(Some(message)) =
        choice(listener.try_next(), sleep(Duration::from_secs(3000))).await?
    {
        info!("{:?}", message)
    }

    let _ = listener.await.trace_error();

    info!("Finished");
    Ok(())
}
