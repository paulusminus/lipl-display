use crate::Interfaces;
use crate::Result;
use crate::error::ErrInto;
use crate::object_path_extensions::OwnedObjectPathExtensions;
use zbus::{
    Connection, export::async_trait::async_trait, fdo::ObjectManagerProxy,
    zvariant::OwnedObjectPath,
};

// Predicate for filtering gatt capable adapters
fn gatt_capable(item: &(OwnedObjectPath, Interfaces)) -> bool {
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
    /// Query Object manager of org.bluez to find adapters
    /// Returns: the first advertising and gatt application capable adapter or Error
    async fn first_gatt_capable_adapter(&self) -> Result<OwnedObjectPath> {
        let proxy = ObjectManagerProxy::builder(self)
            .destination("org.bluez")?
            .path("/")?
            .build()
            .await?;
        let managed_objects = proxy.get_managed_objects().await?;

        managed_objects
            .into_iter()
            .filter(gatt_capable)
            .map(|s| s.0)
            .map(|o| o.as_str().to_owned())
            .min()
            .ok_or(zbus::Error::Unsupported)
            .map(|s| s.to_owned_object_path())
            .err_into()
    }
}

#[cfg(test)]
mod tests {
    use super::ConnectionExt;
    use zbus::Connection;

    #[tokio::test]
    async fn test_first_gatt_capable_adapter() {
        let connection = Connection::system().await.unwrap();
        assert_eq!(connection.is_bus(), true);
        let path = connection.first_gatt_capable_adapter().await.unwrap();
        assert_eq!(path.as_str(), "/org/bluez/hci0")
    }
}
