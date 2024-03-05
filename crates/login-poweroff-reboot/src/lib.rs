pub use dbus::Error;
use std::string::ToString;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use strum::Display;

mod login;

const DESTINATION: &str = "org.freedesktop.login1";
const PATH: &str = "/org/freedesktop/login1";
const TIMEOUT_SECONDS: u64 = 5;

#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum Shutdown {
    Poweroff,
    Reboot,
}

fn time(delay_millis: u64) -> Result<u64, SystemTimeError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|now| now.as_millis() as u64 + delay_millis)
}

pub fn shutdown(shutdown: Shutdown) -> impl Fn(u64) -> Result<(), Error> {
    move |delay_milliseconds| {
        use login::OrgFreedesktopLogin1Manager;

        let connection = dbus::blocking::LocalConnection::new_system()?;
        let proxy = connection.with_proxy(
            DESTINATION,
            PATH,
            std::time::Duration::from_secs(TIMEOUT_SECONDS),
        );
        if let Ok(millis_since_epoch) = time(delay_milliseconds) {
            proxy.schedule_shutdown(&shutdown.to_string(), millis_since_epoch)?;
        };

        Ok(())
    }
}
