// This code was autogenerated with `dbus-codegen-rust -s -d org.freedesktop.login1 -p /org/freedesktop/login1 -f org.freedesktop.login1.Manager`, see https://github.com/diwic/dbus-rs
use dbus::blocking;

pub trait OrgFreedesktopLogin1Manager {
    fn schedule_shutdown(&self, type_: &str, usec: u64) -> Result<(), dbus::Error>;
    // fn cancel_scheduled_shutdown(&self) -> Result<bool, dbus::Error>;
}

impl<T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgFreedesktopLogin1Manager
    for blocking::Proxy<'_, C>
{
    fn schedule_shutdown(&self, type_: &str, usec: u64) -> Result<(), dbus::Error> {
        self.method_call(
            "org.freedesktop.login1.Manager",
            "ScheduleShutdown",
            (type_, usec),
        )
    }

    // fn cancel_scheduled_shutdown(&self) -> Result<bool, dbus::Error> {
    //     self.method_call(
    //         "org.freedesktop.login1.Manager",
    //         "CancelScheduledShutdown",
    //         (),
    //     )
    //     .map(|r: (bool,)| r.0)
    // }
}
