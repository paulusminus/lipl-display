use login_poweroff_reboot::{shutdown, Shutdown};

fn main() -> Result<(), dbus::Error> {
    shutdown(Shutdown::Poweroff)(500)
}
