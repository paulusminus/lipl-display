use futures_util::stream::SelectAll;
use tokio::signal::unix::signal;
use tokio_stream::wrappers::SignalStream;

use crate::error::{ErrInto, Error};

pub use tokio::signal::unix::SignalKind;

pub const INTERRUPT: SignalKind = SignalKind::interrupt();
pub const TERMINATE: SignalKind = SignalKind::terminate();

pub fn combine_signals<S>(signals: S) -> Result<SelectAll<SignalStream>, Error>
where
    S: IntoIterator<Item = SignalKind>,
{
    signals
        .into_iter()
        .map(signal_stream)
        .collect::<Result<Vec<_>, Error>>()
        .map(SelectAll::from_iter)
}

fn signal_stream(signal_kind: SignalKind) -> Result<SignalStream, Error> {
    signal(signal_kind).map(SignalStream::new).err_into()
}
