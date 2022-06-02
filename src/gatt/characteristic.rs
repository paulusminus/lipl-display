use std::{sync::RwLock, collections::HashMap};

use uuid::Uuid;
use zbus::{dbus_interface, zvariant::{OwnedObjectPath, Value}};

#[derive(Debug)]
pub struct Characteristic {
    pub uuid: Uuid,
    pub read: bool,
    pub write: bool,
    pub notify: bool,
    pub service_path: String,
    pub descriptor_paths: Vec<String>,
    pub value: RwLock<String>,
}

impl Characteristic {
    pub fn new_write_only(uuid: Uuid, service_path: String) -> Self {
        Self {
            uuid,
            read: false,
            write: true,
            notify: false,
            service_path,
            descriptor_paths: vec![],
            value: RwLock::new(String::new()),
        }
    }
}

#[dbus_interface(name = "org.bluez.GattCharacteristic1")]
impl Characteristic {

    #[dbus_interface(property)]
    fn descriptors(&self) -> Vec<OwnedObjectPath> {
        self.descriptor_paths
            .clone()
            .into_iter()
            .map(|s| OwnedObjectPath::try_from(s).unwrap())
            .collect()
    }

    #[dbus_interface(property)]
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

    #[dbus_interface(property)]
    fn service(&self) -> OwnedObjectPath {
        OwnedObjectPath::try_from(self.service_path.as_str()).unwrap()
    }

    #[dbus_interface(property, name = "UUID")]
    fn uuid(&self) -> String {
        self.uuid.to_string().to_uppercase()
    }

    #[dbus_interface(name = "WriteValue")]
    fn write_value(&mut self, value: Vec<u8>, _options: HashMap<String, Value>) -> zbus::fdo::Result<()> {
        let s = std::str::from_utf8(&value).map_err(|_| zbus::fdo::Error::IOError("conversion failed".into()))?;
        self.set_value(s.to_owned());
        log::info!("Characteristic {} received {}", self.uuid, s);
        Ok(())
    }

    #[dbus_interface(property = "Value")]
    fn value(&self) -> String {
        let locked_value = self.value.read().unwrap();
        locked_value.clone()
    }

    #[dbus_interface(property = "Value")]
    fn set_value(&mut self, value: String) {
        let mut locked_value = self.value.write().unwrap();
        *locked_value = value;
    }
}