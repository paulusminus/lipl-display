mod login;

const DESTINATION: &str = "org.freedesktop.login1";
const PATH: &str = "/org/freedesktop/login1";
const TIMEOUT_SECONDS: u64 = 5;

pub fn poweroff() -> Result<(), dbus::Error> {
    use login::OrgFreedesktopLogin1Manager;
    let connection = dbus::blocking::LocalConnection::new_system()?;
    let proxy = connection.with_proxy(DESTINATION, PATH, std::time::Duration::from_secs(TIMEOUT_SECONDS));
    proxy.power_off(false)?;

    Ok(())
}

pub fn reboot() -> Result<(), dbus::Error> {
    use login::OrgFreedesktopLogin1Manager;
    let connection = dbus::blocking::LocalConnection::new_system()?;
    let proxy = connection.with_proxy(DESTINATION, PATH, std::time::Duration::from_secs(TIMEOUT_SECONDS));
    proxy.reboot(false)?;

    Ok(())
}
