use std::collections::hash_map::RandomState;
use std::pin::Pin;
use std::sync::Arc;
use std::{collections::HashMap, vec};

use advertisement::PeripheralAdvertisement;
use connection_extension::ConnectionExt;
use futures_channel::mpsc;
use futures_util::{FutureExt, Stream, StreamExt, TryFutureExt};
use gatt::{Application, Request};
pub use lipl_display_common::{BackgroundThread, Command, Message};
use object_path_extensions::OwnedObjectPathExtensions;
use pin_project::{pin_project, pinned_drop};
use proxy::{Adapter1Proxy, Device1Proxy, GattManager1Proxy, LEAdvertisingManager1Proxy};
use tokio::runtime::Handle;
use tracing::{debug, error};
use zbus::fdo::ObjectManagerProxy;
use zbus::{
    Connection,
    names::OwnedInterfaceName,
    object_server::Interface,
    zvariant::{OwnedObjectPath, OwnedValue},
};

use crate::gatt::{Characteristic, Service};
use crate::gatt_application::GattApplication;
use crate::message_handler::{characteristics_map, handle_write_request};

mod advertisement;
mod connection_extension;
#[allow(non_snake_case)]
mod error;
mod gatt;
mod gatt_application;
mod message_handler;
mod object_path_extensions;
pub(crate) mod proxy;

use gatt_application::{
    GattApplicationConfig, GattApplicationConfigBuilder, GattCharacteristicConfig,
    GattCharacteristicConfigBuilder, GattServiceConfigBuilder,
};
type Interfaces =
    HashMap<OwnedInterfaceName, HashMap<String, OwnedValue, RandomState>, RandomState>;

pub use error::{CommonError, Error, Result};

#[pin_project(PinnedDrop)]
pub struct MessageStream<'a> {
    values_tx: mpsc::Sender<Message>,
    #[pin]
    values_rx: mpsc::Receiver<Message>,
    connection: Option<PeripheralConnection<'a>>,
    application_objectpath: OwnedObjectPath,
    advertisement_objectpath: OwnedObjectPath,
}

#[pinned_drop]
impl<'a> PinnedDrop for MessageStream<'a> {
    fn drop(self: Pin<&mut Self>) {
        let this = self.project();
        if let Some(connection) = this.connection.take() {
            let result = Handle::current().block_on(async {
                connection
                    .gatt_manager_proxy
                    .unregister_application(&this.application_objectpath)
                    .await?;
                connection
                    .advertising_manager_proxy
                    .unregister_advertisement(&this.advertisement_objectpath)
                    .await?;
                Ok::<_, zbus::Error>(())
            });
            if let Err(err) = result {
                tracing::error!(
                    "Failed to unregister application and advertisement: {}",
                    err
                );
            }
        };
    }
}

impl<'a> Stream for MessageStream<'a> {
    type Item = Message;
    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().values_rx.poll_next(cx)
    }
}

pub struct ListenZbus<'a> {
    connection: PeripheralConnection<'a>,
}

impl<'a> ListenZbus<'a> {
    pub async fn listen_background(
        &mut self,
        cb: impl Fn(Message) + Send + 'static,
    ) -> Result<
        impl FnOnce() -> Pin<Box<dyn Future<Output = std::result::Result<(), zbus::fdo::Error>> + Send>>,
    > {
        debug!("Establishing connection");
        let bluez = PeripheralConnection::new()
            .await
            .map_err(|_| lipl_display_common::Error::BluetoothAdapter)?;
        debug!("Connection established");

        let (rx, dispose) = bluez
            .run(message_handler::gatt_application_config().unwrap())
            .map_err(|_| lipl_display_common::Error::BluetoothAdapter)
            .await?;

        debug!("Advertising and Gatt application started");

        debug!("Press <Ctr-C> or send signal SIGINT to end service");

        let mut map = characteristics_map();

        let mut receiver = Box::pin(rx);
        while let Some(request) = receiver.next().await {
            if let Request::Write(mut write_request) = request {
                if let Some(message) = handle_write_request(&mut write_request, &mut map) {
                    cb(message.clone());
                    if message == Message::Command(Command::Exit)
                        || message == Message::Command(Command::Poweroff)
                    {
                        break;
                    }
                }
            }
        }
        // dispose()
        //     .await
        //     .map_err(|_| lipl_display_common::Error::Cancelled)?;
        Ok(move || {
            async move {
                dispose().await?;
                Ok::<(), zbus::fdo::Error>(())
            }
            .boxed()
        })
    }
}

impl BackgroundThread for ListenZbus {
    fn stop(&mut self) {}
}

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
                debug!(
                    "{} {} {} from object server",
                    $name,
                    $path,
                    if removed {
                        "removed"
                    } else {
                        "could not be removed"
                    }
                );
            }
            Err(error) => {
                error!("{} {}: {}", $name, $path, error);
            }
        };
    };
}

macro_rules! add_to_server {
    ($server:expr, $object:expr, $hm:expr, $interface_name:literal, $connection:expr, $t:ident) => {
        $server
            .at($object.object_path.clone(), $object.clone())
            .await?;
        let op = $object.object_path.as_str();
        debug!("Service {op} added to object manager");
        let props = $object
            .get_all(
                $server,
                $connection,
                None,
                $server
                    .interface::<_, $t>($object.object_path.clone())
                    .await?
                    .signal_emitter(),
            )
            .await?;
        $hm.insert(
            $object.object_path.to_owned_object_path(),
            vec![($interface_name.to_owned(), props)]
                .into_iter()
                .collect(),
        );
    };
}

impl<'a> PeripheralConnection<'a> {
    fn gatt_manager(&'a self) -> &'a GattManager1Proxy<'a> {
        &self.gatt_manager_proxy
    }

    pub async fn object_manager(&'a self) -> zbus::Result<ObjectManagerProxy<'a>> {
        self.connection.object_manager_proxy().await
    }

    pub fn connection(&'a self) -> &'a Connection {
        &self.connection
    }

    pub async fn device(&'a self, path: &'a str) -> zbus::Result<Device1Proxy<'a>> {
        Device1Proxy::builder(&self.connection)
            .destination("org.bluez")?
            .path(path)?
            .build()
            .await
    }

    /// Creates a dbus connection to bluez
    /// Finds the first gatt capable adapter
    /// Set adapter powered and discoverable1
    pub async fn new() -> zbus::Result<PeripheralConnection<'a>> {
        let connection = Connection::system().await?;

        debug!("Connected to session dbus");
        let adapter = connection.first_gatt_capable_adapter().await?;
        debug!("First capapable adapter: {}", adapter.as_str());

        let adapter_proxy = Adapter1Proxy::builder(&connection)
            .destination("org.bluez")?
            .path(adapter.clone())?
            .build()
            .await?;
        debug!("Adapter proxy created");

        let gatt_manager_proxy = GattManager1Proxy::builder(&connection)
            .destination("org.bluez")?
            .path(adapter.clone())?
            .build()
            .await?;
        debug!("Gatt manager proxy created");

        let advertising_manager_proxy = LEAdvertisingManager1Proxy::builder(&connection)
            .destination("org.bluez")?
            .path(adapter.clone())?
            .build()
            .await?;
        debug!("Advertising manager proxy created");

        adapter_proxy.set_powered(true).await?;
        debug!("Set powered to true");
        adapter_proxy.set_discoverable(true).await?;
        debug!("Set discoverable to true");

        let name = adapter_proxy.name().await?;
        let address = adapter_proxy.address().await?;
        let path = adapter_proxy.inner().path().as_str();
        debug!("Adapter {path} with address {address} on {name}");

        Ok(Self {
            connection,
            gatt_manager_proxy,
            advertising_manager_proxy,
            adapter_proxy,
        })
    }

    pub fn adapter(&'a self) -> Adapter1Proxy<'a> {
        self.adapter_proxy.clone()
    }

    /// Run a gatt application with advertising
    pub async fn run(
        &'a self,
        gatt_application_config: gatt_application::GattApplicationConfig,
    ) -> zbus::Result<MessageStream<'a>> {
        let (tx, rx) = mpsc::channel::<Request>(10);
        let object_server = self.connection.object_server();
        let gatt_application: GattApplication = (gatt_application_config, tx).into();

        // Advertising
        let advertisement = PeripheralAdvertisement::from(&gatt_application);
        let advertisement_path =
            format!("{}/advertisement", gatt_application.app_object_path).to_owned_object_path();
        let advertising_proxy = self.advertising_manager_proxy.clone();
        object_server.at(&advertisement_path, advertisement).await?;
        debug!(
            "Advertisement {} added to object server",
            advertisement_path.as_str()
        );
        self.advertising_manager_proxy
            .register_advertisement(&advertisement_path, HashMap::new())
            .await?;
        debug!(
            "Advertisement {} registered with bluez",
            advertisement_path.as_str()
        );

        // Gatt application
        let mut hm: HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>> =
            HashMap::new();

        for service in gatt_application.services.clone() {
            add_to_server!(
                object_server,
                service,
                hm,
                "org.bluez.GattService1",
                &self.connection,
                Service
            );
        }

        for characteristic in gatt_application.characteristics.clone() {
            add_to_server!(
                object_server,
                characteristic,
                hm,
                "org.bluez.GattCharacteristic1",
                &self.connection,
                Characteristic
            );
        }

        let app = Application { objects: hm };
        let app_object_path = gatt_application
            .clone()
            .app_object_path
            .to_owned_object_path();
        object_server.at(&app_object_path, app.clone()).await?;
        let app_op = gatt_application.app_object_path.clone();
        debug!("Application {app_op} added to object server");

        self.gatt_manager()
            .register_application(&app_object_path, HashMap::new())
            .await?;
        debug!("Application {app_op} registered with bluez");
        let gatt_manager_proxy = self.gatt_manager().clone();

        let application = gatt_application;

        Ok(MessageStream {
            values_tx: tx,
            values_rx: rx,
            connection: (),
            application_objectpath: app_object_path,
            advertisement_objectpath: advertisement_path,
        })
    }
}

#[cfg(test)]
mod tests {}
