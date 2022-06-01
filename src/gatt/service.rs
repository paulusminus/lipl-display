use uuid::Uuid;
use zbus::dbus_interface;

pub struct Service {
    pub primary: bool,
    pub uuid: Uuid,
    pub characteristic_paths: Vec<String>,
}

#[dbus_interface(name = "org.bluez.GattService1")]
impl Service {
    #[dbus_interface(property = "Primary")]
    fn primary(&self) -> bool {
        self.primary
    }

    #[dbus_interface(property = "UUID")]
    fn uuid(&self) -> String {
        self.uuid.to_string()
    }

    #[dbus_interface(property = "Characteristics")]
    fn characteristics(&self) -> Vec<String> {
        self.characteristic_paths.clone()
    }

}