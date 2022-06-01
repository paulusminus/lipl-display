use std::collections::HashMap;
use uuid::Uuid;
use zbus::{dbus_interface};


pub struct PeripheralAdvertisement {
    pub service_uuids: Vec<Uuid>,
    pub manufacturer_data: HashMap<u16, Vec<u8>>,
    pub local_name: String,
    pub include_tx_power: bool,
}

fn manufacturer_data() -> HashMap<u16, Vec<u8>> {
    let mut hm = HashMap::new();
    hm.insert(0xFFFF, "PM".as_bytes().to_vec());
    hm
}

impl PeripheralAdvertisement {
    pub fn new(local_name: String, services: Vec<Uuid>) -> Self {
        Self { 
            service_uuids: services,
            manufacturer_data: manufacturer_data(),
            local_name: local_name,
            include_tx_power: true,
        }
    }
}

#[dbus_interface(name = "org.bluez.LEAdvertisement1")]
impl PeripheralAdvertisement {
    #[dbus_interface(property = "Type")]
    fn advertisement_type(&self) -> String {
        "peripheral".into()
    }

    #[dbus_interface(property = "ManufacturerData")]
    fn manufacturer_data(&self) -> HashMap<u16, zbus::zvariant::Value> {
        self
            .manufacturer_data
            .clone()
            .into_iter()
            .map(|s| (s.0, zbus::zvariant::Value::from(s.1)))
            .collect()
    }

    #[dbus_interface(property = "ServiceUUIDs")]
    fn service_uuids(&self) -> Vec<String> {
        self.service_uuids.iter().map(|uuid| uuid.to_string()).collect()
    }

    #[dbus_interface(property = "LocalName")]
    fn local_name(&self) -> String {
        self.local_name.clone()
    }

    #[dbus_interface(property = "IncludeTxPower")]
    fn include_tx_power(&self) -> bool {
        self.include_tx_power
    }

    fn release(&self) {
        log::info!("Released");
    }
}
