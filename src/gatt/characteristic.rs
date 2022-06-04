use std::{sync::{RwLock, Arc}, collections::HashMap};
use futures_channel::{mpsc::Sender};

use uuid::Uuid;
use zbus::{dbus_interface, zvariant::{OwnedObjectPath, Value}};
use crate::object_path_extensions::OwnedObjectPathExtensions;

#[derive(Clone, Debug)]
pub struct Characteristic {
    pub object_path: String,
    pub uuid: Uuid,
    pub read: bool,
    pub write: bool,
    pub notify: bool,
    pub service_path: String,
    pub descriptor_paths: Vec<String>,
    pub value: Arc<RwLock<String>>,
    pub sender: Sender<(Uuid, String)>,
}

pub struct WriteOptions {
    mtu: Option<u16>,
    device: Option<String>,
    offset: Option<u16>,
}

macro_rules! option_convert {
    ($option:expr, $key:literal, $output:ty, $variant:path, $convert:ident) => {
        $option.get($key).and_then(|option| {
            match option {
                $variant(value) => Some(value.$convert()),
                _ => None,
            }
        })
        
    };
}

impl From<&HashMap<String, Value<'_>>> for WriteOptions {
    fn from(options: &HashMap<String, Value>) -> Self {
        Self { 
            mtu: option_convert!(options, "mtu", u16, Value::U16, clone),
            device: option_convert!(options, "device", String, Value::ObjectPath, to_string),
            offset: option_convert!(options, "offset", u16, Value::U16, clone),
        }
    }
}

impl std::fmt::Display for WriteOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = vec![];
        if let Some(mtu) = self.mtu {
            v.push(format!("mtu: {}", mtu));
        }
        if let Some(device) = &self.device {
            v.push(format!("device: {}", device));
        }
        if let Some(offset) = self.offset {
            v.push(format!("offset: {}", offset));
        }
        write!(f, "{}", v.join(", "))
    }
}

impl Characteristic {
    pub fn new_write_only(object_path: String, uuid: Uuid, service_path: String, sender: Sender<(Uuid, String)>) -> Self {
        Self {
            object_path,
            uuid,
            read: false,
            write: true,
            notify: false,
            service_path,
            descriptor_paths: vec![],
            value: Arc::new(RwLock::new(String::new())),
            sender,
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
            .map(|s| s.to_owned_object_path())
            .collect()
    }

    #[dbus_interface(property)]
    fn flags(&self) -> Vec<String> {
        let mut flags = vec![];
        if self.read {
            flags.push("read".to_owned());
        }
        if self.write {
            flags.push("write-without-response".to_owned());
        }
        flags
    }

    #[dbus_interface(property)]
    fn service(&self) -> OwnedObjectPath {
        self.service_path.to_owned_object_path()
    }

    #[dbus_interface(property, name = "UUID")]
    fn uuid(&self) -> String {
        self.uuid.to_string().to_uppercase()
    }

    #[dbus_interface(name = "WriteValue")]
    fn write_value(&mut self, value: Vec<u8>, options: HashMap<String, Value>) -> zbus::fdo::Result<()> {
        let s = std::str::from_utf8(&value).map_err(|_| zbus::fdo::Error::IOError("conversion failed".into()))?;
        self.set_value(s.to_owned());

        log::info!("Characteristic {} write with data {}", self.uuid, s);

        let write_options: WriteOptions = (&options).into();
        log::info!("Write options: {}", write_options);
        self
            .sender
            .try_send((self.uuid, s.to_owned()))
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string())) 
    }   

    #[dbus_interface(property = "Value")]
    fn value(&self) -> String {
        let locked_value = self.value.read().unwrap();
        locked_value.clone()
    }

    #[dbus_interface(property = "Value")]
    fn set_value(&self, value: String) {
        let mut locked_value = self.value.write().unwrap();
        *locked_value = value;
    }
}