use async_trait::async_trait;
use zbus::fdo::ObjectManagerProxy;
use zbus::zvariant::OwnedObjectPath;
use zbus::Result;
use zbus::Connection;
use crate::Interfaces;

// Predicate for filtering gatt capable adapters
pub fn gatt_capable(item: &(OwnedObjectPath, Interfaces)) -> bool {
    item.1.contains_key("org.bluez.Adapter1")
    && item.1.contains_key("org.bluez.GattManager1") 
    && item.1.contains_key("org.bluez.LEAdvertisingManager1")
}

#[async_trait]
pub trait ConnectionExt {
    async fn first_gatt_capable_adapter(&self) -> Result<OwnedObjectPath>;
}

#[async_trait]
impl ConnectionExt for Connection {
    /// Query Object manager of org.bluze to find adapters
    /// Returns: the first advertising and gatt application capable adapter or Error
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
