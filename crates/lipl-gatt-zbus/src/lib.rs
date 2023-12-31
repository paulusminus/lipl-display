use std::{collections::HashMap, vec};
use std::collections::hash_map::RandomState;
use std::pin::Pin;

use advertisement::PeripheralAdvertisement;
use proxy::{Adapter1Proxy, Device1Proxy, GattManager1Proxy, LEAdvertisingManager1Proxy};
use async_channel::{unbounded, Receiver};
use zbus::export::futures_util::{TryFutureExt, StreamExt};
use gatt::{Application, Request};
pub use lipl_display_common::{Message, Listen, Command};
use zbus::fdo::ObjectManagerProxy;
use zbus::{
    Connection,
    ConnectionBuilder,
    names::OwnedInterfaceName,
    export::{
        futures_core::future::Future,
        futures_util::FutureExt,
    },
    zvariant::{
        OwnedObjectPath,
        OwnedValue,
    },
    Interface,
};
use connection_extension::ConnectionExt;
use object_path_extensions::OwnedObjectPathExtensions;

use crate::gatt::{Service, Characteristic};
use crate::gatt_application::GattApplication;
use crate::message_handler::{characteristics_map, handle_write_request};

mod advertisement;
pub(crate) mod proxy;
mod connection_extension;
#[allow(non_snake_case)]
mod error;
mod gatt;
mod gatt_application;
mod message_handler;
mod object_path_extensions;

use gatt_application::{GattApplicationConfig, GattApplicationConfigBuilder, GattServiceConfigBuilder, GattCharacteristicConfigBuilder, GattCharacteristicConfig};
type Interfaces = HashMap<OwnedInterfaceName, HashMap<String, OwnedValue, RandomState>, RandomState>;

pub use error::{Error, Result, CommonError};

pub struct ListenZbus;

impl Listen for ListenZbus {
    fn listen_background(&mut self, cb: impl Fn(Message) + Send + 'static) {
        std::thread::spawn(move || {
            async_io::block_on(async move {
                let bluez =
                    PeripheralConnection::new()
                    .await
                    .map_err(|_| lipl_display_common::Error::BluetoothAdapter)?;
    
                let (mut rx, dispose) = 
                    bluez
                    .run(message_handler::gatt_application_config().unwrap())
                    .map_err(|_| lipl_display_common::Error::BluetoothAdapter)
                    .await?;
    
                tracing::info!("Advertising and Gatt application started");
            
                tracing::info!("Press <Ctr-C> or send signal SIGINT to end service");
            
                let mut map = characteristics_map();
    
                while let Some(request) = rx.next().await {
                    if let Request::Write(mut write_request) = request {
                        if let Some(message) = handle_write_request(&mut write_request, &mut map) {
                            cb(message.clone());
                            if message == Message::Command(Command::Exit) || message == Message::Command(Command::Poweroff) {
                                break;
                            }        
                        }
                    }
                }
                dispose().await.map_err(|_| lipl_display_common::Error::Cancelled)?;
                Ok::<(), lipl_display_common::Error>(())
            })
        });            
    }

    fn stop(&mut self) {
        
    }
}

// pub fn listen_background(cb: impl Fn(Message) -> lipl_display_common::Result<()> + Send + 'static) {
//     std::thread::spawn(move || {
//         async_io::block_on(async move {
//             let bluez =
//                 PeripheralConnection::new()
//                 .await
//                 .map_err(|_| lipl_display_common::Error::BluetoothAdapter)?;

//             let (mut rx, dispose) = 
//                 bluez
//                 .run(message_handler::gatt_application_config().unwrap())
//                 .map_err(|_| lipl_display_common::Error::BluetoothAdapter)
//                 .await?;

//             tracing::info!("Advertising and Gatt application started");
        
//             tracing::info!("Press <Ctr-C> or send signal SIGINT to end service");
        
//             let mut map = characteristics_map();

//             while let Some(request) = rx.next().await {
//                 if let Request::Write(mut write_request) = request {
//                     if let Some(message) = handle_write_request(&mut write_request, &mut map) {
//                         cb(message.clone()).map_err(|_| lipl_display_common::Error::Callback)?;
//                         if message == Message::Command(Command::Exit) || message == Message::Command(Command::Poweroff) {
//                             break;
//                         }        
//                     }
//                 }
//             }
//             dispose().await.map_err(|_| lipl_display_common::Error::Cancelled)?;
//             Ok::<(), lipl_display_common::Error>(())
//         })
//     });
// }


pub struct PeripheralConnection<'a> {
    connection: Connection,
    gatt_manager_proxy: GattManager1Proxy<'a>,
    advertising_manager_proxy: LEAdvertisingManager1Proxy<'a>,
    adapter_proxy: Adapter1Proxy<'a>,
}

macro_rules! remove_from_server {
    ($server:expr, $type:ty, $path:expr, $name:literal) => {
        match $server.remove::<$type, _>($path).await {
            Ok(removed) => {
                tracing::info!("{} {} {} from object server", $name, $path, if removed { "removed"} else {"could not be removed"});
            },
            Err(error) => {
                tracing::error!("{} {}: {}", $name, $path, error);
            }
        };
    };
}

macro_rules! add_to_server {
    ($server:expr, $object:expr, $hm:expr, $interface_name:literal) => {
        $server.at($object.object_path.clone(), $object.clone()).await?;
        let op = $object.object_path.as_str();
        tracing::info!("Service {op} added to object manager");
        let props = $object.get_all().await;
        $hm.insert(
            $object.object_path.to_owned_object_path(),
            vec![
                ($interface_name.to_owned(), props)
            ]
            .into_iter()
            .collect()
        );
    };            
}


impl<'a> PeripheralConnection<'a> {
    fn gatt_manager(&'a self) -> &'a GattManager1Proxy {
        &self.gatt_manager_proxy
    }

    pub async fn object_manager(&'a self) -> zbus::Result<ObjectManagerProxy<'a>> {
        self.connection.object_manager_proxy().await
    }

    pub fn connection(&'a self) -> &'a Connection {
        &self.connection
    }

    pub async fn device(&'a self, path: &'a str) -> zbus::Result<Device1Proxy<'a>> {
        Device1Proxy::builder(&self.connection).destination("org.bluez")?.path(path)?.build().await
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
        let path = adapter_proxy.path().as_str();
        tracing::info!("Adapter {path} with address {address} on {name}");

        Ok( 
            Self { 
                connection,
                gatt_manager_proxy,
                advertising_manager_proxy,
                adapter_proxy,
            }
        )
    }

    pub fn adapter(&'a self) -> Adapter1Proxy<'a> {
        self.adapter_proxy.clone()
    }

    /// Run a gatt application with advertising
    pub async fn run(&'a self, gatt_application_config: gatt_application::GattApplicationConfig) ->
    zbus::Result<(Receiver<Request>, impl FnOnce() -> Pin<Box<(dyn Future<Output = zbus::fdo::Result<()>> + 'a + Send)>>)>
    {
        let (tx, rx) = unbounded::<Request>();
        let object_server = self.connection.object_server();
        let gatt_application: GattApplication = (gatt_application_config, tx).into();

        // Advertising
        let advertisement = PeripheralAdvertisement::from(&gatt_application);
        let advertisement_path = format!("{}/advertisement", gatt_application.app_object_path).to_owned_object_path();
        let advertising_proxy = self.advertising_manager_proxy.clone();
        object_server.at(&advertisement_path, advertisement).await?;
        tracing::info!("Advertisement {} added to object server", advertisement_path.as_str());
        self.advertising_manager_proxy
            .register_advertisement(
                &advertisement_path,
                HashMap::new(),
            )
            .await?;
        tracing::info!("Advertisement {} registered with bluez", advertisement_path.as_str());

        // Gatt application
        let mut hm: HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>> = HashMap::new();

        for service in gatt_application.services.clone() {
            add_to_server!(object_server, service, hm, "org.bluez.GattService1");
        }

        for characteristic in gatt_application.characteristics.clone() {
            add_to_server!(object_server, characteristic, hm, "org.bluez.GattCharacteristic1");
        }

        let app = Application {
            objects: hm,
        };
        let app_object_path = gatt_application.clone().app_object_path.to_owned_object_path();
        object_server
            .at(&app_object_path, app.clone())
            .await?;
        let app_op = gatt_application.app_object_path.clone();
        tracing::info!("Application {app_op} added to object server");

        self
            .gatt_manager()
            .register_application(
                &app_object_path,
                HashMap::new(),
            )
            .await?;
        tracing::info!("Application {app_op} registered with bluez");
        let gatt_manager_proxy = self.gatt_manager().clone();

        let application = gatt_application;

        Ok(
            ( 
                rx,
                || async move {
                    gatt_manager_proxy.unregister_application(&app_object_path).await?;
                    tracing::info!("Application {app_op} unregistered with bluez");

                    advertising_proxy.unregister_advertisement(&advertisement_path).await?;
                    tracing::info!("Advertisement {} unregistered with bluez", advertisement_path.as_str());

                    for characteristic in application.characteristics.clone() {
                        remove_from_server!(object_server, Characteristic, characteristic.object_path.as_str(), "Characteristic");
                    }

                    for service in application.services.clone() {
                        remove_from_server!(object_server, Service, service.object_path.as_str(), "Service");
                    }

                    remove_from_server!(object_server, PeripheralAdvertisement, advertisement_path.as_str(), "Advertisement");
                    remove_from_server!(object_server, Application, app_object_path.as_str(), "Application");

                    Ok::<(), zbus::fdo::Error>(())
                }
                .boxed(),
            )
        )
    }
}


#[cfg(test)]
mod tests {}
