use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::pin::Pin;

use advertisement::PeripheralAdvertisement;
use async_trait::async_trait;
use bluez_interfaces::{Adapter1Proxy, LEAdvertisingManager1Proxy, GattManager1Proxy};
use gatt::Application;
use zbus::export::futures_util::FutureExt;
use zbus::names::OwnedInterfaceName;
use zbus::zvariant::{OwnedObjectPath, OwnedValue};
use zbus::{fdo::ObjectManagerProxy, Connection, ConnectionBuilder};
use zbus::{Result};
use zbus::export::futures_core::future::Future;

pub use gatt::SERVICE_1_UUID;

pub mod advertisement;
pub mod bluez_interfaces;
mod gatt;

pub type Interfaces = HashMap<OwnedInterfaceName, HashMap<String, OwnedValue, RandomState>, RandomState>;
pub const ADVERTISEMENT_PATH: &str = "/org/bluez/advertisement";

#[async_trait]
pub trait ConnectionExt {
    async fn first_gatt_capable_adapter(&self) -> Result<OwnedObjectPath>;
}

#[async_trait]
impl ConnectionExt for Connection {
    async fn first_gatt_capable_adapter(&self) -> zbus::Result<OwnedObjectPath> {
        let proxy = ObjectManagerProxy::builder(self).destination("org.bluez")?.path("/")?.build().await?;
        let managed_objects = proxy.get_managed_objects().await?;

        let adapters = 
            managed_objects
            .into_iter()
            .filter(gatt_capable)
            .map(|s| s.0)
            .collect::<Vec<OwnedObjectPath>>();
        
        adapters
        .into_iter()
        .map(|o| o.as_str().to_owned()).min().ok_or_else(|| zbus::Error::Unsupported)
        .map(|s| OwnedObjectPath::try_from(s).unwrap())
    }
}

pub struct BluezDbusConnection {
    connection: Connection,
    adapter: OwnedObjectPath,
}

impl BluezDbusConnection {
    pub async fn new() -> zbus::Result<Self> {
        let connection = 
            ConnectionBuilder::system()?
            .build()
            .await?;
        let adapter = connection.first_gatt_capable_adapter().await?;
        let bluez_dbus_connection = Self { connection, adapter };
        bluez_dbus_connection.power_on().await?;
        Ok(bluez_dbus_connection)
    }

    pub async fn adapter_proxy<'a>(&'a self) -> zbus::Result<Adapter1Proxy<'a>> {
        Adapter1Proxy::builder(&self.connection).destination("org.bluez")?.path(&self.adapter)?.build().await
    }

    pub async fn gatt_manager_proxy<'a>(&'a self) -> zbus::Result<GattManager1Proxy<'a>> {
        GattManager1Proxy::builder(&self.connection).destination("org.bluez")?.path(&self.adapter)?.build().await
    }

    pub async fn advertising_manager_proxy<'a>(&'a self) -> zbus::Result<LEAdvertisingManager1Proxy<'a>> {
        LEAdvertisingManager1Proxy::builder(&self.connection).destination("org.bluez")?.path(&self.adapter)?.build().await
    }

    pub async fn power_on(&self) -> zbus::Result<()> {
        let proxy = self.adapter_proxy().await?;
        proxy.set_powered(true).await?;
        proxy.set_discoverable(true).await
    }

    pub async fn register_application(&self) -> zbus::Result<Application> {
        gatt::register_application(&self.connection, &self.adapter).await
    }

    pub async fn register_advertisement<'a>(&'a self, advertisement: PeripheralAdvertisement) ->
    zbus::Result<impl FnOnce() -> Pin<Box<(dyn Future<Output = zbus::fdo::Result<()>> + 'a + Send)>>>
    {
        let advertisement_path = OwnedObjectPath::try_from(ADVERTISEMENT_PATH).unwrap();
        let proxy = self.advertising_manager_proxy().await?;
        let connection = self.connection.clone();
        connection.object_server().at(&advertisement_path, advertisement).await?;
        log::info!("Advertisement registered with objectserver at {}", ADVERTISEMENT_PATH);
        proxy
            .register_advertisement(
                &advertisement_path,
                HashMap::new(),
            )
            .await?;
        log::info!("Advertisement {} registered with bluez", ADVERTISEMENT_PATH);

        Ok(
            || async move {
                proxy.unregister_advertisement(&advertisement_path).await?;
                log::info!("Advertisement {} unregistered with bluez", ADVERTISEMENT_PATH);
                connection.object_server().remove::<PeripheralAdvertisement, &OwnedObjectPath>(&advertisement_path).await?;
                log::info!("Advertisement {} removed from objectserver", ADVERTISEMENT_PATH);
                Ok::<(), zbus::fdo::Error>(())
            }
            .boxed()
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
