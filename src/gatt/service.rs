use uuid::Uuid;
use zbus::{dbus_interface, zvariant::OwnedObjectPath};

#[derive(Clone, Debug)]
pub struct Service {
    pub primary: bool,
    pub uuid: Uuid,
    pub characteristic_paths: &'static[&'static str],
}

#[dbus_interface(name = "org.bluez.GattService1")]
impl Service {
    #[dbus_interface(property)]
    fn primary(&self) -> bool {
        self.primary
    }

    #[dbus_interface(property, name = "UUID")]
    fn uuid(&self) -> String {
        self.uuid.to_string().to_uppercase()
    }

    #[dbus_interface(property)]
    fn characteristics(&self) -> Vec<OwnedObjectPath> {
        self.characteristic_paths.into_iter().cloned().map(|s| OwnedObjectPath::try_from(s).unwrap()).collect()
    }
}