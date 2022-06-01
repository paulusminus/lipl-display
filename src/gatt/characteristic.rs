use std::collections::HashMap;

use uuid::Uuid;
use zbus::{dbus_interface};

pub struct Characteristic {
    pub uuid: Uuid,
    pub read: bool,
    pub write: bool,
    pub notify: bool,
    pub service_path: String,
    pub descriptor_paths: Vec<String>,
    pub value: String,
}

#[dbus_interface(name = "org.bluez.GattCharacteristic1")]
impl Characteristic {

    #[dbus_interface(property = "Descriptors")]
    fn descriptors(&self) -> Vec<String> {
        self.descriptor_paths.clone()
    }

    #[dbus_interface(property = "Flags")]
    fn flags(&self) -> Vec<String> {
        let mut flags = vec![];
        if self.read {
            flags.push("read".to_owned());
        }
        if self.write {
            flags.push("write".to_owned());
        }
        flags
    }

    #[dbus_interface(property = "Service")]
    fn service(&self) -> String {
        self.service_path.clone()
    }

    #[dbus_interface(property = "UUID")]
    fn uuid(&self) -> String {
        self.uuid.to_string()
    }

    #[dbus_interface(name = "WriteValue")]
    fn write_value(&mut self, value: Vec<u8>, _options: HashMap<String, zbus::zvariant::Value>) -> zbus::fdo::Result<()> {
        let s = std::str::from_utf8(&value).map_err(|_| zbus::fdo::Error::IOError("conversion failed".into()))?;
        self.set_value(s.to_owned());
        Ok(())
    }

    #[dbus_interface(property = "Value")]
    fn value(&self) -> String {
        self.value.clone()
    }

    #[dbus_interface(property = "Value")]
    fn set_value(&mut self, value: String) {
        self.value = value;
    }
}