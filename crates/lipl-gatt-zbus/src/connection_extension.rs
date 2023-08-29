use zbus::{
    Connection,
    Result,
    fdo::ObjectManagerProxy,
    export::async_trait::async_trait,
    zvariant::OwnedObjectPath,
};
use crate::Interfaces;
use crate::object_path_extensions::OwnedObjectPathExtensions;

// Predicate for filtering gatt capable adapters
pub fn gatt_capable(item: &(OwnedObjectPath, Interfaces)) -> bool {
    item.1.contains_key("org.bluez.Adapter1")
    && item.1.contains_key("org.bluez.GattManager1") 
    && item.1.contains_key("org.bluez.LEAdvertisingManager1")
}

#[async_trait]
pub trait ConnectionExt {
    async fn first_gatt_capable_adapter(&self) -> Result<OwnedObjectPath>;
    async fn object_manager_proxy(&self) -> Result<ObjectManagerProxy>;
}

#[async_trait]
impl ConnectionExt for Connection {
    /// Query Object manager of org.bluze to find adapters
    /// Returns: the first advertising and gatt application capable adapter or Error
    async fn first_gatt_capable_adapter(&self) -> zbus::Result<OwnedObjectPath> {
        let proxy = ObjectManagerProxy::builder(self).destination("org.bluez")?.path("/")?.build().await?;
        let managed_objects = proxy.get_managed_objects().await?;

        managed_objects
        .into_iter()
        .filter(gatt_capable)
        .map(|s| s.0)
        .map(|o| o.as_str().to_owned()).min().ok_or(zbus::Error::Unsupported)
        .map(|s| s.to_owned_object_path())
    }

    async fn object_manager_proxy(&self) -> Result<ObjectManagerProxy> {
        ObjectManagerProxy::builder(self).destination("org.bluez")?.path("/")?.build().await
    }
}
