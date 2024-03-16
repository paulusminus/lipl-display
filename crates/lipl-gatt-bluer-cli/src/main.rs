use futures_util::{pin_mut, StreamExt};
use lipl_display_common::{Command, Message};
use lipl_gatt_bluer::listen_stream;
use login_poweroff_reboot::{shutdown, Shutdown};

use error::{ErrInto, Error};
use signal::{combine_signals, SignalKind, INTERRUPT, TERMINATE};

mod error;
mod out;
mod signal;

const EXIT_ON_SIGNALS: [SignalKind; 2] = [INTERRUPT, TERMINATE];

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let mut out = out::Out::default();

    let stream = listen_stream().await.map(|s| s.fuse())?;
    pin_mut!(stream);

    let combined_signal = combine_signals(EXIT_ON_SIGNALS)?;
    pin_mut!(combined_signal);

    loop {
        tokio::select! {
            _ = combined_signal.next() => { break; }
            message = stream.select_next_some() => {
                out.send_json(&message)?;
                if message == Message::Command(Command::Poweroff) {
                    shutdown(Shutdown::Poweroff)(1000).err_into()?;
                    break;
                } else if message == Message::Command(Command::Exit) {
                    break;
                }
            }
        }
    }

    Ok(())
}
