use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::future::Future;

use zbus::export::futures_util::{TryFutureExt};
use zbus::names::OwnedInterfaceName;
use zbus::zvariant::{OwnedObjectPath, OwnedValue};
use zbus::{fdo::ObjectManagerProxy, Connection};
use zbus::Result;

pub mod bluez_interfaces;

pub type Interfaces = HashMap<OwnedInterfaceName, HashMap<String, OwnedValue, RandomState>, RandomState>;

pub struct Bluez {
    connection: Connection,
}

impl Bluez {
    pub fn new() -> impl Future<Output = Result<Self>> {
        Connection::system().map_ok(|connection| Self { connection})
    }

    pub async fn bluez_managed_objects(&self) -> Result<HashMap<OwnedObjectPath, Interfaces>> {
        let proxy = ObjectManagerProxy::builder(&self.connection).destination("org.bluez")?.path("/")?.build().await?;
        let om = proxy.get_managed_objects().await?;
        Ok(om)
    }

    pub async fn list_adapters(&self, filter: impl Fn(&(OwnedObjectPath, Interfaces)) -> bool) -> Result<impl Iterator<Item = OwnedObjectPath>> {
        self.bluez_managed_objects()
        .await
        .map(|hm| 
            hm
            .into_iter()
            .filter(filter)
            .map(|s| s.0)
        )
    }    
}


pub fn gatt_capable(item: &(OwnedObjectPath, Interfaces)) -> bool {
    item.1.contains_key("org.bluez.Adapter1")
    && item.1.contains_key("org.bluez.GattManager1") 
    && item.1.contains_key("org.bluez.LEAdvertisingManager1")
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
