use login_poweroff_reboot::{Shutdown, shutdown};

fn main() -> Result<(), dbus::Error> {
    shutdown(Shutdown::Poweroff)(500)
}
