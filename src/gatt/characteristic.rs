use std::collections::HashMap;

use uuid::Uuid;
use zbus::{dbus_interface, zvariant::OwnedObjectPath};

#[derive(Clone, Debug)]
pub struct Characteristic {
    pub uuid: Uuid,
    pub read: bool,
    pub write: bool,
    pub notify: bool,
    pub service_path: &'static str,
    pub descriptor_paths: Vec<String>,
    pub value: String,
}

#[dbus_interface(name = "org.bluez.GattCharacteristic1")]
impl Characteristic {

    #[dbus_interface(property = "Descriptors")]
    fn descriptors(&self) -> Vec<OwnedObjectPath> {
        self.descriptor_paths
            .clone()
            .into_iter()
            .map(|s| OwnedObjectPath::try_from(s).unwrap())
            .collect()
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
    fn service(&self) -> OwnedObjectPath {
        OwnedObjectPath::try_from(self.service_path).unwrap()
    }

    #[dbus_interface(property = "UUID")]
    fn uuid(&self) -> String {
        self.uuid.to_string().to_uppercase()
    }

    #[dbus_interface(name = "WriteValue")]
    fn write_value(&mut self, value: Vec<u8>, _options: HashMap<String, zbus::zvariant::Value>) -> zbus::fdo::Result<()> {
        let s = std::str::from_utf8(&value).map_err(|_| zbus::fdo::Error::IOError("conversion failed".into()))?;
        log::info!("Characteristic {} received {}", self.uuid, s);
        // self.set_value(s.to_owned());
        Ok(())
    }

    // #[dbus_interface(property = "Value")]
    // fn value(&self) -> String {
    //     self.value.clone()
    // }

    // #[dbus_interface(property = "Value")]
    // fn set_value(&mut self, value: String) {
    //     self.value = value;
    // }
}