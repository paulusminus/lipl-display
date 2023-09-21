use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

mod login;

const DESTINATION: &str = "org.freedesktop.login1";
const PATH: &str = "/org/freedesktop/login1";
const POWEROFF: &str = "poweroff";
const REBOOT: &str = "reboot";
const TIMEOUT_SECONDS: u64 = 5;

fn time(delay_millis: u64) -> Result<u64, SystemTimeError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|now| now.as_millis() as u64 + delay_millis)
}

/// Call poweroff method on logind dbus interface
pub fn poweroff(delay_milliseconds: u64) -> Result<(), dbus::Error> {
    use login::OrgFreedesktopLogin1Manager;
    let connection = dbus::blocking::LocalConnection::new_system()?;
    let proxy = connection.with_proxy(
        DESTINATION,
        PATH,
        std::time::Duration::from_secs(TIMEOUT_SECONDS),
    );

    if let Ok(millis_since_epoch) = time(delay_milliseconds) {
        proxy.schedule_shutdown(POWEROFF, millis_since_epoch)?;
    };

    Ok(())
}

/// Call reboot method on logind dbus interface
pub fn reboot(delay_milliseconds: u64) -> Result<(), dbus::Error> {
    use login::OrgFreedesktopLogin1Manager;
    let connection = dbus::blocking::LocalConnection::new_system()?;
    let proxy = connection.with_proxy(
        DESTINATION,
        PATH,
        std::time::Duration::from_secs(TIMEOUT_SECONDS),
    );
    if let Ok(millis_since_epoch) = time(delay_milliseconds) {
        proxy.schedule_shutdown(REBOOT, millis_since_epoch)?;
    };

    Ok(())
}
