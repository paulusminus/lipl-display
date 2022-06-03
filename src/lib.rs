use std::{collections::HashMap, vec};
use std::collections::hash_map::RandomState;
use std::pin::Pin;

use advertisement::PeripheralAdvertisement;
use adapter_interfaces::{Adapter1Proxy, LEAdvertisingManager1Proxy, GattManager1Proxy};
use futures_channel::mpsc::Receiver;
use gatt::Application;
use uuid::Uuid;
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

use crate::gatt::{Service, Characteristic};
use crate::gatt_application::GattApplication;

mod advertisement;
pub(crate) mod adapter_interfaces;
mod connection_extension;
mod gatt;
mod gatt_application;

pub use gatt_application::{GattApplicationConfig, GattServiceConfig, GattCharacteristicConfig};
type Interfaces = HashMap<OwnedInterfaceName, HashMap<String, OwnedValue, RandomState>, RandomState>;

pub struct PeripheralConnection<'a> {
    connection: Connection,
    gatt_manager_proxy: GattManager1Proxy<'a>,
    advertising_manager_proxy: LEAdvertisingManager1Proxy<'a>,
}

impl<'a> PeripheralConnection<'a> {
    fn gatt_manager(&'a self) -> &'a GattManager1Proxy {
        &self.gatt_manager_proxy
    }

    /// Creates a dbus connection to bluez 
    /// Finds the first gatt capable adapter
    /// Set adapter powered and discoverable1
    pub async fn new() -> zbus::Result<PeripheralConnection<'a>> {
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

    /// Run a gatt application with advertising
    pub async fn run(&'a self, gatt_application_config: gatt_application::GattApplicationConfig) ->
    zbus::Result<(Receiver<(Uuid, String)>, impl FnOnce() -> Pin<Box<(dyn Future<Output = zbus::fdo::Result<()>> + 'a + Send)>>)>
    {
        let (tx, rx) = futures_channel::mpsc::channel::<(Uuid, String)>(10);
        let object_server = self.connection.object_server();
        let gatt_application: GattApplication = (gatt_application_config, tx).into();

        // Advertising
        let advertisement = PeripheralAdvertisement::from(&gatt_application);
        let advertisement_path = OwnedObjectPath::try_from(format!("{}/advertisement", gatt_application.app_object_path).as_str()).unwrap();
        let advertising_proxy = self.advertising_manager_proxy.clone();
        object_server.at(&advertisement_path, advertisement).await?;
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
            object_server.at(service.object_path.clone(), service.clone()).await?;
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
            object_server.at(characteristic.object_path.clone(), characteristic.clone()).await?;
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
        object_server
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
            ( 
                rx,
                || async move {
                    gatt_manager_proxy.unregister_application(&app_object_path).await?;
                    log::info!("Application {} unregistered with bluez", app_object_path.as_str());
                    advertising_proxy.unregister_advertisement(&advertisement_path).await?;
                    log::info!("Advertisement {} unregistered with bluez", advertisement_path.as_str());

                    let removed_message = |removed: bool| if removed { "removed"} else {"could not be removed"};

                    if let Ok(removed) = object_server.remove::<Application, _>(&app_object_path).await {
                        log::info!("Application {} {} from object server", app_object_path.as_str(), removed_message(removed));
                    };

                    for service in application.services.clone() {
                        match object_server.remove::<Service, _>(service.object_path.as_str()).await {
                            Ok(removed) => {
                                log::info!("Service {} {} from object server", service.object_path.as_str(), removed_message(removed));
                            },
                            Err(error) => {
                                log::error!("Service {}: {}", service.object_path.as_str(), error);
                            }
                        };
                    }

                    for characteristic in application.characteristics.clone() {
                        if let Ok(removed) = object_server.remove::<Characteristic, _>(characteristic.object_path.as_str()).await {
                            log::info!("Characteristic {} {} from object server", characteristic.object_path.as_str(), removed_message(removed));
                        }
                    }

                    log::info!("Service interface name: {}", Service::name().as_str());
                    if let Ok(removed) = object_server.remove::<PeripheralAdvertisement, _>(&advertisement_path).await {
                        log::info!("Advertisement {} {} from objectserver", advertisement_path.as_str(), removed_message(removed));
                    }
                    Ok::<(), zbus::fdo::Error>(())
                }
                .boxed(),
            )
        )
    }
}


#[cfg(test)]
mod tests {}
