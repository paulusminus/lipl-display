use std::collections::HashMap;

use zbus::{dbus_interface, zvariant::OwnedValue};

pub struct Advertisement {
    pub advertisement_type: String,
    pub service_uuids: Vec<String>,
    pub manufacturer_data: HashMap<String, OwnedValue>,
    pub sollicit_uuids: Vec<String>,
    pub name: String,
    pub include_tx_power: bool,
}

#[dbus_interface(name = "org.bluez.LEAdvertisement1")]
impl Advertisement {
    #[dbus_interface(property = "Type")]
    async fn advertisement_type(&self) -> String {
        self.advertisement_type.clone()
    }

    #[dbus_interface(property = "ServiceUUIDs")]
    async fn service_uuids(&self) -> Vec<String> {
        self.service_uuids.clone()
    }

    #[dbus_interface(property = "ManufacturerData")]
    async fn manufacturer_data(&self) -> HashMap<String, OwnedValue> {
        self.manufacturer_data.clone()
    }

    #[dbus_interface(property = "SollictitUUIDs")]
    async fn sollictit_uuids(&self) -> Vec<String> {
        self.sollicit_uuids.clone()
    }

    #[dbus_interface(property = "Name")]
    async fn name(&self) -> String {
        self.name.clone()
    }

    #[dbus_interface(property = "IncludeTxPower")]
    async fn include_tx_power(&self) -> bool {
        self.include_tx_power
    }



}
