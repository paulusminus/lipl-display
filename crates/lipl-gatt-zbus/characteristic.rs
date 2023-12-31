//! # DBus interface proxy for: `org.bluez.GattCharacteristic1`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Interface '/org/bluez/hci0/dev_43_45_C0_00_1F_AC/service0015/char0018' from service 'org.bluez' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.bluez.GattCharacteristic1")]
trait GattCharacteristic1 {
    /// AcquireNotify method
    fn acquire_notify(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<(zbus::zvariant::OwnedFd, u16)>;

    /// AcquireWrite method
    fn acquire_write(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<(zbus::zvariant::OwnedFd, u16)>;

    /// ReadValue method
    fn read_value(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<Vec<u8>>;

    /// StartNotify method
    fn start_notify(&self) -> zbus::Result<()>;

    /// StopNotify method
    fn stop_notify(&self) -> zbus::Result<()>;

    /// WriteValue method
    fn write_value(
        &self,
        value: &[u8],
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Flags property
    #[dbus_proxy(property)]
    fn flags(&self) -> zbus::Result<Vec<String>>;

    /// MTU property
    #[dbus_proxy(property)]
    fn mtu(&self) -> zbus::Result<u16>;

    /// NotifyAcquired property
    #[dbus_proxy(property)]
    fn notify_acquired(&self) -> zbus::Result<bool>;

    /// Notifying property
    #[dbus_proxy(property)]
    fn notifying(&self) -> zbus::Result<bool>;

    /// Service property
    #[dbus_proxy(property)]
    fn service(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// UUID property
    #[dbus_proxy(property)]
    fn uuid(&self) -> zbus::Result<String>;

    /// Value property
    #[dbus_proxy(property)]
    fn value(&self) -> zbus::Result<Vec<u8>>;

    /// WriteAcquired property
    #[dbus_proxy(property)]
    fn write_acquired(&self) -> zbus::Result<bool>;
}
