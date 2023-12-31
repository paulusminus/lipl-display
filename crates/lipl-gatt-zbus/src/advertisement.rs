use std::{collections::HashMap, vec};
use uuid::Uuid;
use zbus::dbus_interface;

use crate::gatt_application::GattApplication;
const MANUFACTURER_NAME: &str = "PM";

#[derive(Debug)]
pub struct PeripheralAdvertisement {
    pub service_uuids: Vec<Uuid>,
    pub manufacturer_data: HashMap<u16, Vec<u8>>,
    pub local_name: String,
    pub include_tx_power: bool,
}

fn unregistered_manufacturer_data() -> HashMap<u16, Vec<u8>> {
    let mut hm = HashMap::new();
    hm.insert(0xFFFF, MANUFACTURER_NAME.as_bytes().to_vec());
    hm
}

impl Default for PeripheralAdvertisement {
    fn default() -> Self {
        Self { 
            service_uuids: vec![], 
            manufacturer_data: unregistered_manufacturer_data(), 
            local_name: "".into(), 
            include_tx_power: true 
        }
    }
}

impl From<&GattApplication> for PeripheralAdvertisement {
    fn from(gatt_application: &GattApplication) -> Self {
        Self { 
            service_uuids: gatt_application.services.iter().map(|service| service.uuid).collect(), 
            local_name: gatt_application.local_name.clone(), 
            ..Default::default()
        }
    }
}

#[dbus_interface(name = "org.bluez.LEAdvertisement1")]
impl PeripheralAdvertisement {
    #[dbus_interface(property, name = "Type")]
    fn advertisement_type(&self) -> String {
        "peripheral".into()
    }

    #[dbus_interface(property, name = "ManufacturerData")]
    fn manufacturer_data(&self) -> HashMap<u16, zbus::zvariant::Value> {
        self
            .manufacturer_data
            .clone()
            .into_iter()
            .map(|s| (s.0, zbus::zvariant::Value::from(s.1)))
            .collect()
    }

    #[dbus_interface(property, name = "ServiceUUIDs")]
    fn service_uuids(&self) -> Vec<String> {
        self.service_uuids.iter().map(|uuid| uuid.to_string().to_uppercase()).collect()
    }

    #[dbus_interface(property)]
    fn local_name(&self) -> String {
        self.local_name.clone()
    }

    #[dbus_interface(property)]
    fn include_tx_power(&self) -> bool {
        self.include_tx_power
    }

    fn release(&self) {
        tracing::info!("Released");
    }
}
