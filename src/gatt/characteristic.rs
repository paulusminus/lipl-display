use std::{collections::HashMap};
use async_channel::{bounded, Sender};

use uuid::Uuid;
use zbus::{dbus_interface, zvariant::{OwnedObjectPath, Value}};
use crate::{object_path_extensions::OwnedObjectPathExtensions, GattCharacteristicConfig};

#[derive(Clone, Debug)]
pub struct Characteristic {
    pub object_path: String,
    pub uuid: Uuid,
    pub read: bool,
    pub write: bool,
    pub notify: bool,
    pub service_path: String,
    pub descriptor_paths: Vec<String>,
    pub sender: Sender<Request>,
    pub service_uuid: Uuid,
}

impl From<(usize, &GattCharacteristicConfig, String, Sender<Request>, Uuid)> for Characteristic {
    fn from(gatt_char_config: (usize, &GattCharacteristicConfig, String, Sender<Request>, Uuid)) -> Self {
        Characteristic::new(
            format!("{}/char{}", gatt_char_config.2, gatt_char_config.0 + 1),
            gatt_char_config.1.uuid,
            gatt_char_config.2.clone(),
            gatt_char_config.3.clone(),
            gatt_char_config.1.read,
            gatt_char_config.1.write,
            gatt_char_config.4,
        )

    }
}

#[derive(Debug)]
pub struct WriteRequest {
    pub uuid: Uuid,
    pub value: Vec<u8>,
    pub mtu: Option<u16>,
    pub device: Option<String>,
    pub offset: Option<u16>,
    pub write_type: Option<String>,
    pub service_uuid: Uuid,
}

#[derive(Debug)]
pub struct ReadRequest {
    pub uuid: Uuid,
    pub mtu: Option<u16>,
    pub device: Option<String>,
    pub offset: Option<u16>,
    pub sender: Option<Sender<Vec<u8>>>,
    pub service_uuid: Uuid,
}

#[derive(Debug)]
pub enum Request {
    Read(ReadRequest),
    Write(WriteRequest),
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

impl From<(Uuid, Vec<u8>, &HashMap<String, Value<'_>>, Uuid)> for WriteRequest {
    fn from(options: (Uuid, Vec<u8>, &HashMap<String, Value>, Uuid)) -> Self {
        WriteRequest {
            uuid: options.0,
            value: options.1,
            mtu: option_convert!(options.2, "mtu", u16, Value::U16, clone),
            device: option_convert!(options.2, "device", String, Value::ObjectPath, to_string),
            offset: option_convert!(options.2, "offset", u16, Value::U16, clone),
            write_type: option_convert!(options.2, "type", String, Value::Str, to_string),
            service_uuid: options.3,
        }
    }
}

impl From<(Uuid, &HashMap<String, Value<'_>>, Sender<Vec<u8>>, Uuid)> for ReadRequest {
    fn from(options: (Uuid, &HashMap<String, Value<'_>>, Sender<Vec<u8>>, Uuid)) -> Self {
        ReadRequest {
            uuid: options.0,
            mtu: option_convert!(options.1, "mtu", u16, Value::U16, clone),
            device: option_convert!(options.1, "device", String, Value::ObjectPath, to_string),
            offset: option_convert!(options.1, "offset", u16, Value::U16, clone),
            sender: Some(options.2),
            service_uuid: options.3,
        }
    }
}

fn option_display<T: std::fmt::Display>(name: &str, option: &Option<T>) -> Option<String> {
    option.as_ref().map(|v| format!("{name}: {v}"))
}

struct VecU8<'a>(&'a Vec<u8>);

impl<'a> VecU8<'a> {
    fn display(&'a self) -> Option<&str> {
        std::str::from_utf8(self.0.as_slice()).ok()
    }
}

trait Joiner {
    fn join(&self, seperator: &'static str) -> String;
}

impl Joiner for [Option<String>] {
    fn join(&self, seperator: &'static str) -> String {
        self.iter().flatten().cloned().collect::<Vec<_>>().join(seperator)
    }
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Request::Write(request) => {
                write!(
                    f,
                    "{}",
                    [
                        option_display("operation", &Some("write")),
                        option_display("mtu", &request.mtu),
                        option_display("device", &request.device),
                        option_display("offset", &request.offset),
                        option_display("value", &VecU8(&request.value).display()),
                    ]
                    .join(", ")
                )
            },
            Request::Read(request) => {
                write!(
                    f,
                    "{}",
                    [
                        option_display("operation", &Some("read")),
                        option_display("mtu", &request.mtu),
                        option_display("device", &request.device),
                        option_display("offset", &request.offset),
                    ]
                    .join(", ")
                )
            },
        }
    }
}

impl Characteristic {
    pub fn new(
        object_path: String,
        uuid: Uuid,
        service_path: String,
        sender: Sender<Request>,
        read: bool,
        write: bool,
        service_uuid: Uuid,
    ) -> Self {
        Self {
            object_path,
            uuid,
            read,
            write,
            notify: false,
            service_path,
            descriptor_paths: vec![],
            sender,
            service_uuid,
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
            flags.push("encrypt-authenticated-read".into());
        }
        if self.write {
            flags.push("encrypt-authenticated-write".to_owned());
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

    #[dbus_interface(name = "ReadValue")]
    async fn read_value(&mut self, options: HashMap<String, Value<'_>>) -> zbus::fdo::Result<Vec<u8>> {
        if !self.read { return Err(zbus::fdo::Error::NotSupported("org.bluez.Error.NotSupported".into())); }
        let (tx, rx) = bounded::<Vec<u8>>(1);
        let read_request: ReadRequest = (self.uuid, &options, tx, self.service_uuid).into();
        self.sender
            .try_send(Request::Read(read_request))
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;
        let result = rx.recv().await.map_err(|error| zbus::fdo::Error::IOError(error.to_string()))?;
        Ok(result)
    }

    #[dbus_interface(name = "WriteValue")]
    fn write_value(&mut self, value: Vec<u8>, options: HashMap<String, Value>) -> zbus::fdo::Result<()> {
        if !self.write { return Err(zbus::fdo::Error::NotSupported("org.bluez.Error.NotSupported".into())); }
        let write_request: WriteRequest = (self.uuid, value, &options, self.service_uuid).into();
        self
            .sender
            .try_send(Request::Write(write_request))
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string())) 
    }   

}