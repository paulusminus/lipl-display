//! Library to send messages through dbus to Login Manager


mod login_manager;

use std::time::{SystemTime, UNIX_EPOCH};
use zbus::blocking::{Connection};

use login_manager::ManagerProxyBlocking;

const LOGIN: &str = "org.freedeskop.login1";
const PATH: &str = "/org/freedesktop/login1";
const POWEROFF: &str = "poweroff";
const REBOOT: &str = "reboot";

fn time(delay_millis: u64) -> zbus::Result<u64> {
    SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .map(|now| now.as_millis() as u64 + delay_millis)
    .map_err(|_| zbus::Error::Unsupported)
}

fn manager(connection: &Connection) -> zbus::Result<ManagerProxyBlocking<'_>> {
    login_manager::ManagerProxyBlocking::builder(connection)
        .destination(LOGIN)?
        .path(PATH)?
        .build()
}

fn shutdown(delay_milliseconds: u64, shutdown_type: &str) -> zbus::Result<()> {
    time(delay_milliseconds)
    .and_then(
        |millis_since_epoch| 
            manager(&Connection::system()?)?
            .schedule_shutdown(shutdown_type, millis_since_epoch)
    )
}

/// Poweroff machine the program is running on
/// 
/// ## Example
/// 
/// ```
/// use login_poweroff_reboot_zbus::poweroff;
/// if let Ok(_) = poweroff(1000) {
///   println!("Command to poweroff machine was sent");
/// }
/// ```
/// 
pub fn poweroff(delay_milliseconds: u64) -> zbus::Result<()> {
    shutdown(delay_milliseconds, POWEROFF)
}

/// Reboot machine the program is running on
/// 
/// ## Example
/// 
/// ```
/// use login_poweroff_reboot_zbus::reboot;
/// if let Ok(_) = reboot(1000) {
///   println!("Command to reboot machine was sent");
/// }
/// ```
/// 
pub fn reboot(delay_milliseconds: u64) -> zbus::Result<()> {
    shutdown(delay_milliseconds, REBOOT)
}
