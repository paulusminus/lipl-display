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
    #[dbus_interface(property = "Primary")]
    fn primary(&self) -> bool {
        self.primary
    }

    #[dbus_interface(property = "UUID")]
    #[allow(non_snake_case)]
    fn UUID(&self) -> String {
        log::info!("Service UUID: {}", self.uuid.to_string().to_uppercase());
        self.uuid.to_string().to_uppercase()
    }

    #[dbus_interface(property = "Characteristics")]
    fn characteristics(&self) -> Vec<OwnedObjectPath> {
        self.characteristic_paths.into_iter().cloned().map(|s| OwnedObjectPath::try_from(s).unwrap()).collect()
    }
}