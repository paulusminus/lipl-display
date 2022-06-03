use std::{collections::HashMap, vec};
use std::collections::hash_map::RandomState;
use std::pin::Pin;

use advertisement::PeripheralAdvertisement;
use adapter_interfaces::{Adapter1Proxy, LEAdvertisingManager1Proxy, GattManager1Proxy};
use gatt::Application;
use zbus::{
    Connection,
    ConnectionBuilder,
    names::{
        OwnedInterfaceName,
    },
    export::{
        futures_core::{
            future::{
                Future,
            },
        },
        futures_util::FutureExt,
    },
    zvariant::{
        OwnedObjectPath,
        OwnedValue,
    }, Interface,
};
use connection_extension::ConnectionExt;

use crate::gatt_application::GattApplication;

// use crate::gatt::{Characteristic, Service};

mod advertisement;
pub(crate) mod adapter_interfaces;
mod connection_extension;
mod gatt;
mod gatt_application;

pub use gatt_application::{GattApplicationConfig, GattServiceConfig, GattCharacteristicConfig};

type Interfaces = HashMap<OwnedInterfaceName, HashMap<String, OwnedValue, RandomState>, RandomState>;
// pub const ADVERTISEMENT_PATH: &str = "/org/bluez/advertisement";

pub struct BluezDbusConnection<'a> {
    connection: Connection,
    gatt_manager_proxy: GattManager1Proxy<'a>,
    advertising_manager_proxy: LEAdvertisingManager1Proxy<'a>,
}

impl<'a> BluezDbusConnection<'a> {
    fn gatt_manager(&'a self) -> &'a GattManager1Proxy {
        &self.gatt_manager_proxy
    }

    /// Creates a connection. 
    /// Finds the first capable adapter
    /// Initialize proxies to interfaces
    pub async fn new() -> zbus::Result<BluezDbusConnection<'a>> {
        let connection = 
            ConnectionBuilder::system()?
            .build()
            .await?;

        let adapter =
            connection
            .first_gatt_capable_adapter()
            .await?;

        let adapter_proxy = 
            Adapter1Proxy::builder(&connection)
            .destination("org.bluez")?
            .path(adapter.clone())?
            .build()
            .await?;

        let gatt_manager_proxy =
            GattManager1Proxy::builder(&connection)
            .destination("org.bluez")?
            .path(adapter.clone())?
            .build()
            .await?;

        let advertising_manager_proxy =
            LEAdvertisingManager1Proxy::builder(&connection)
            .destination("org.bluez")?
            .path(adapter.clone())?
            .build()
            .await?;

        adapter_proxy.set_powered(true).await?;
        adapter_proxy.set_discoverable(true).await?;
        let name = adapter_proxy.name().await?;
        let address = adapter_proxy.address().await?;
        let path = adapter_proxy.path();
        log::info!("Adapter {} with address {} on {}", path.as_str(), address, name);

        Ok( 
            Self { 
                connection,
                gatt_manager_proxy,
                advertising_manager_proxy,
            }
        )
    }

    pub async fn run(&'a self, gatt_application_config: gatt_application::GattApplicationConfig) ->
    zbus::Result<impl FnOnce() -> Pin<Box<(dyn Future<Output = zbus::fdo::Result<()>> + 'a + Send)>>>
    {
        let gatt_application: GattApplication = gatt_application_config.into();
        // Advertising
        let advertisement: PeripheralAdvertisement = gatt_application.clone().into();
        let advertisement_path = OwnedObjectPath::try_from(format!("{}/advertisement", gatt_application.app_object_path).as_str()).unwrap();
        let connection = self.connection.clone();
        let advertising_proxy = self.advertising_manager_proxy.clone();
        connection.object_server().at(&advertisement_path, advertisement).await?;
        log::info!("Advertisement {} added to object server", advertisement_path.as_str());
        self.advertising_manager_proxy
            .register_advertisement(
                &advertisement_path,
                HashMap::new(),
            )
            .await?;
        log::info!("Advertisement {} registered with bluez", advertisement_path.as_str());

        // Gatt application
        let mut hm: HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>> = HashMap::new();

        for service in gatt_application.services.clone() {
            connection.object_server().at(service.object_path.clone(), service.clone()).await?;
            log::info!("Service {} added to object manager", service.object_path);
            let service_props = service.get_all().await;
            hm.insert(
                OwnedObjectPath::try_from(service.object_path.as_str()).unwrap(),
                vec![
                    ("org.bluez.GattService1".to_owned(), service_props)
                ]
                .into_iter()
                .collect()
            );
        }

        for characteristic in gatt_application.characteristics.clone() {
            connection.object_server().at(characteristic.object_path.clone(), characteristic.clone()).await?;
            log::info!("Characteristic {} added to object server", characteristic.object_path);
            let char_props = characteristic.get_all().await;
            hm.insert(
                OwnedObjectPath::try_from(characteristic.object_path.as_str()).unwrap(),
                vec![
                    ("org.bluez.GattCharacteristic1".to_owned(), char_props)
                ]
                .into_iter()
                .collect(),
            );
        }

        let app = Application {
            objects: hm,
        };
        let app_object_path = OwnedObjectPath::try_from(gatt_application.clone().app_object_path.as_str()).unwrap();
        connection
            .object_server()
            .at(&app_object_path, app.clone())
            .await?;
        log::info!("Application {} added to object server", gatt_application.app_object_path);

        self
            .gatt_manager()
            .register_application(
                &app_object_path,
                HashMap::new(),
            )
            .await?;
        log::info!("Application {} registered with bluez", gatt_application.app_object_path);
        let gatt_manager_proxy = self.gatt_manager().clone();
        // let gatt_manager_path = OwnedObjectPath::try_from(gatt_application.app_object_path.as_str()).unwrap();

        let application = gatt_application;

        Ok(
            || async move {
                gatt_manager_proxy.unregister_application(&app_object_path).await?;
                log::info!("Application {} unregistered with bluez", app_object_path.as_str());
                connection.object_server().remove::<Application, &OwnedObjectPath>(&app_object_path).await?;
                log::info!("Application {} removed from object server", app_object_path.as_str());

                // for service in application.services.clone() {
                //     let object_path = OwnedObjectPath::try_from(service.object_path.as_str()).unwrap();
                //     connection.object_server().remove::<Service, &OwnedObjectPath>(&object_path).await?;
                //     log::info!("Service {} removed from object server", object_path.as_str());
                // }

                // for characteristic in application.characteristics.clone() {
                //     let object_path = OwnedObjectPath::try_from(characteristic.object_path.as_str()).unwrap();
                //     connection.object_server().remove::<Characteristic, &OwnedObjectPath>(&object_path).await?;
                //     log::info!("Characteristic {} removed from object server", object_path.as_str());
                // }

                advertising_proxy.unregister_advertisement(&advertisement_path).await?;
                log::info!("Advertisement {} unregistered with bluez", advertisement_path.as_str());
                connection.object_server().remove::<PeripheralAdvertisement, &OwnedObjectPath>(&advertisement_path).await?;
                log::info!("Advertisement {} removed from objectserver", advertisement_path.as_str());
                Ok::<(), zbus::fdo::Error>(())
            }
            .boxed()
        )
    }
}


#[cfg(test)]
mod tests {}
