use std::collections::HashMap;

use zbus::{dbus_interface, Guid};

pub struct Advertisement {
    pub advertisement_type: String,
    pub service_uuids: Vec<Guid>,
    pub manufacturer_data: HashMap<u16, Vec<u8>>,
    // pub solicit_uuids: Vec<String>,
    pub local_name: String,
    pub include_tx_power: bool,
}

#[dbus_interface(name = "org.bluez.LEAdvertisement1")]
impl Advertisement {
    #[dbus_interface(property = "Type")]
    fn advertisement_type(&self) -> String {
        self.advertisement_type.clone()
    }

    #[dbus_interface(property = "ServiceUUIDs")]
    fn service_uuids(&self) -> Vec<String> {
        self.service_uuids.iter().map(|guid| guid.as_str().to_owned()).collect()
    }

    #[dbus_interface(property = "ManufacturerData")]
    fn manufacturer_data(&self) -> HashMap<u16, Vec<u8>> {
        self.manufacturer_data.clone()
    }

    #[dbus_interface(property = "LocalName")]
    fn name(&self) -> String {
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
