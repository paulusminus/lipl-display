use std::collections::HashMap;
use std::collections::hash_map::RandomState;

use bluez_interfaces::{Adapter1Proxy, LEAdvertisingManager1Proxy};
use zbus::names::OwnedInterfaceName;
use zbus::zvariant::{OwnedObjectPath, OwnedValue};
use zbus::{fdo::ObjectManagerProxy, Connection, ConnectionBuilder};
use zbus::{Result, Interface};

pub mod bluez_interfaces;
pub mod advertisement;

pub type Interfaces = HashMap<OwnedInterfaceName, HashMap<String, OwnedValue, RandomState>, RandomState>;

pub struct BluezDbusConnection {
    connection: Connection,
}

impl BluezDbusConnection {
    pub async fn new(advertisement: (OwnedObjectPath, impl Interface)) -> Result<Self> {
        let connection = 
            ConnectionBuilder::system()?
            .serve_at(&advertisement.0, advertisement.1)?
            .build()
            .await?;
        let bluez_dbus_connection = Self {connection};
        let adapters = bluez_dbus_connection.list_adapters(gatt_capable).await?;
        if let Some(adapter) = adapters.keys().map(|path| path.as_str()).min() {
            log::info!("Found adapter {}", adapter);
            bluez_dbus_connection.power_on(adapter).await?;
            log::info!("Adapter {} powered on and discoverable", adapter);
            bluez_dbus_connection.register_advertisement(&adapter.try_into()?, &advertisement.0).await?;
            log::info!("Adapter {} advertisement registered", adapter);
        }
        Ok(bluez_dbus_connection)
    }

    pub async fn power_on(&self, path: &str) -> Result<()> {
        let proxy: Adapter1Proxy = Adapter1Proxy::builder(&self.connection).destination("org.bluez")?.path(path)?.build().await?;
        proxy.set_powered(true).await?;
        proxy.set_discoverable(true).await
    }

    pub async fn register_advertisement(&self, adapter: &OwnedObjectPath, advertisement: &OwnedObjectPath) -> Result<()> {
        let proxy = LEAdvertisingManager1Proxy::builder(&self.connection).destination("org.bluez")?.path(adapter)?.build().await?;
        proxy.register_advertisement(advertisement, HashMap::new()).await
    }

    pub async fn bluez_managed_objects(&self) -> Result<HashMap<OwnedObjectPath, Interfaces>> {
        let proxy = ObjectManagerProxy::builder(&self.connection).destination("org.bluez")?.path("/")?.build().await?;
        let om = proxy.get_managed_objects().await?;
        Ok(om)
    }

    pub async fn list_adapters(&self, filter: impl Fn(&(OwnedObjectPath, Interfaces)) -> bool) -> Result<HashMap<OwnedObjectPath, Interfaces>> {
        self.bluez_managed_objects()
        .await
        .map(|hm| 
            hm
            .into_iter()
            .filter(filter)
            .collect()
        )
    }
}

pub fn gatt_capable(item: &(OwnedObjectPath, Interfaces)) -> bool {
    item.1.contains_key("org.bluez.Adapter1")
    && item.1.contains_key("org.bluez.GattManager1") 
    && item.1.contains_key("org.bluez.LEAdvertisingManager1")
}

#[cfg(test)]
mod tests {}
