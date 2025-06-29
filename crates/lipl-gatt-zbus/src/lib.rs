use std::collections::hash_map::RandomState;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::{collections::HashMap, vec};

use crate::gatt_application::GattApplication;
use crate::message_handler::{characteristics_map, handle_write_request};
use advertisement::PeripheralAdvertisement;
use connection_extension::ConnectionExt;
use futures::channel::mpsc::{Receiver, channel};
use futures::{FutureExt, Stream, StreamExt, TryFutureExt, select};
use gatt::{Application, Characteristic, Request, Service};
pub use lipl_display_common::{BackgroundThread, Command, Message};
use object_path_extensions::OwnedObjectPathExtensions;
use pin_project::{pin_project, pinned_drop};
use proxy::{Adapter1Proxy, Device1Proxy, GattManager1Proxy, LEAdvertisingManager1Proxy};
use zbus::fdo::{ObjectManagerProxy, PropertiesProxy};
use zbus::names::InterfaceName;
use zbus::{
    ObjectServer,
    connection::{Builder, Connection},
    names::OwnedInterfaceName,
    object_server::Interface,
    zvariant::{OwnedObjectPath, OwnedValue},
};

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
pub struct GattListener {
    task: tokio::task::JoinHandle<()>,
    #[pin]
    receiver: futures::channel::mpsc::Receiver<Message>,
    terminate: Option<futures::channel::oneshot::Sender<()>>,
}

#[pinned_drop]
impl PinnedDrop for GattListener {
    fn drop(self: Pin<&mut Self>) {
        let this = self.project();
        if let Some(terminate) = this.terminate.take() {
            terminate.send(()).ok();
        }
    }
}

impl Stream for GattListener {
    type Item = Message;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().receiver.poll_next(cx)
    }
}

impl Default for GattListener {
    fn default() -> Self {
        Self::new()
    }
}

impl GattListener {
    pub fn new() -> Self {
        let (mut sender, receiver) = futures::channel::mpsc::channel::<Message>(100);
        let (terminate, mut terminate_receiver) = futures::channel::oneshot::channel::<()>();
        Self {
            task: tokio::runtime::Handle::current().spawn(async move {
                let bluez = PeripheralConnection::new()
                    .await
                    .map_err(|_| lipl_display_common::Error::BluetoothAdapter)
                    .expect("problem with bluetooth adapter");

                let (mut rx, dispose) = bluez
                    .run(message_handler::gatt_application_config().unwrap())
                    .map_err(|_| lipl_display_common::Error::BluetoothAdapter)
                    .await
                    .expect("problem with initialization");

                tracing::info!("Advertising and Gatt application started");

                tracing::info!("Press <Ctr-C> or send signal SIGINT to end service");

                let mut map = characteristics_map();

                loop {
                    select! {
                        request = rx.next() => {
                            if let Some(Request::Write(mut write_request)) = request {
                                if let Some(message) = handle_write_request(&mut write_request, &mut map) {
                                    sender.try_send(message.clone()).unwrap();
                                    if message == Message::Command(Command::Exit)
                                        || message == Message::Command(Command::Poweroff)
                                    {
                                        break;
                                    }
                                }
                            }
                        },
                        _ = terminate_receiver => break,
                    }
                }
                dispose.await.expect("Cannot dispose");
            }),
            receiver,
            terminate: Some(terminate),
        }
    }
}

#[derive(Clone)]
pub struct PeripheralConnection {
    connection: Connection,
    adapter: OwnedObjectPath,
}

pub async fn remove_from_server<I: Interface>(object_server: &ObjectServer, object_path: &str) {
    match object_server.remove::<I, _>(object_path).await {
        Ok(removed) => {
            tracing::info!(
                "{} {} from object server",
                object_path,
                if removed {
                    "removed"
                } else {
                    "could not be removed"
                }
            );
        }
        Err(error) => {
            tracing::error!(
                "Failed to remove {} from object server: {}",
                object_path,
                error
            );
        }
    }
}

impl PeripheralConnection {
    pub async fn object_manager_proxy(&self) -> zbus::Result<ObjectManagerProxy<'_>> {
        self.connection.object_manager_proxy().await
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    pub async fn properties(
        &self,
        object_path: OwnedObjectPath,
        interface_name: &'static str,
    ) -> zbus::fdo::Result<HashMap<String, OwnedValue>> {
        self.properties_proxy(object_path)
            .await
            .unwrap()
            .get_all(InterfaceName::from_static_str(interface_name).unwrap())
            .await
    }

    pub async fn device(&self, path: OwnedObjectPath) -> zbus::Result<Device1Proxy<'_>> {
        Device1Proxy::builder(&self.connection)
            .destination("org.bluez")?
            .path(path)?
            .build()
            .await
    }

    pub async fn adapter_proxy(&self) -> zbus::Result<Adapter1Proxy<'_>> {
        Adapter1Proxy::builder(&self.connection)
            .destination("org.bluez")?
            .path(self.adapter.clone())?
            .build()
            .await
    }

    pub async fn gatt_manager_proxy(&self) -> zbus::Result<GattManager1Proxy<'_>> {
        GattManager1Proxy::builder(&self.connection)
            .destination("org.bluez")?
            .path(self.adapter.clone())?
            .build()
            .await
    }

    pub async fn properties_proxy(
        &self,
        path: OwnedObjectPath,
    ) -> zbus::Result<PropertiesProxy<'_>> {
        PropertiesProxy::builder(&self.connection)
            .destination("org.bluez")?
            .path(path)?
            .build()
            .await
    }

    pub async fn advertising_manager_proxy(&self) -> zbus::Result<LEAdvertisingManager1Proxy<'_>> {
        LEAdvertisingManager1Proxy::builder(&self.connection)
            .destination("org.bluez")?
            .path(&self.adapter)?
            .build()
            .await
    }

    /// Creates a dbus connection to bluez
    /// Finds the first gatt capable adapter
    /// Set adapter powered and discoverable1
    pub async fn new() -> zbus::Result<PeripheralConnection> {
        let connection = Builder::system()?.build().await?;

        let adapter = connection.first_gatt_capable_adapter().await?;
        let peripheral_connection = PeripheralConnection {
            connection,
            adapter,
        };

        let adapter_proxy = peripheral_connection.adapter_proxy().await?;

        adapter_proxy.set_powered(true).await?;
        adapter_proxy.set_discoverable(true).await?;

        let name = adapter_proxy.name().await?;
        let address = adapter_proxy.address().await?;
        tracing::info!("Adapter with address {address} and name {name}");

        Ok(peripheral_connection)
    }

    /// Run a gatt application with advertising
    pub async fn run(
        &self,
        gatt_application_config: gatt_application::GattApplicationConfig,
    ) -> zbus::Result<(
        Receiver<Request>,
        Pin<Box<(dyn Future<Output = zbus::fdo::Result<()>> + Send + '_)>>,
    )> {
        let (tx, rx) = channel::<Request>(1);
        let object_server = self.connection.object_server();
        let gatt_application: GattApplication = (gatt_application_config, tx).into();

        // Advertising
        let advertisement = PeripheralAdvertisement::from(&gatt_application);
        let advertisement_path =
            format!("{}/advertisement", gatt_application.app_object_path).to_owned_object_path();
        let advertising_manager_proxy = self.advertising_manager_proxy().await?.clone();
        object_server.at(&advertisement_path, advertisement).await?;
        tracing::info!(
            "Advertisement {} added to object server",
            advertisement_path.as_str()
        );
        advertising_manager_proxy
            .register_advertisement(&advertisement_path, HashMap::new())
            .await?;
        tracing::info!(
            "Advertisement {} registered with bluez",
            advertisement_path.as_str()
        );

        // Gatt application
        let mut hm: HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>> =
            HashMap::new();

        for service in gatt_application.services.clone() {
            let object_path = service.object_path.to_owned_object_path();
            let object_path_clone = object_path.clone();
            tracing::info!(
                "Service {} about te be registered with bluez",
                &object_path_clone
            );
            self.connection()
                .object_server()
                .at(&object_path, service.clone())
                .await?;
            let interface = self
                .connection()
                .object_server()
                .interface::<_, Service>(&object_path)
                .await?;
            let properties = service
                .get_all(
                    self.connection().object_server(),
                    self.connection(),
                    None,
                    interface.signal_emitter(),
                )
                .await?;
            let x = Service::name().as_str().to_owned();
            hm.insert(object_path, vec![(x, properties)].into_iter().collect());
            tracing::info!("Service {} registered with bluez", &object_path_clone);
        }

        for characteristic in gatt_application.characteristics.clone() {
            let object_path = characteristic.object_path.to_owned_object_path();
            let object_path_clone = object_path.clone();
            tracing::info!(
                "Service {} about te be registered with bluez",
                &object_path_clone
            );
            self.connection()
                .object_server()
                .at(&object_path, characteristic.clone())
                .await?;
            let interface = self
                .connection()
                .object_server()
                .interface::<_, Characteristic>(&object_path)
                .await?;
            let properties = characteristic
                .get_all(
                    self.connection().object_server(),
                    self.connection(),
                    None,
                    interface.signal_emitter(),
                )
                .await?;
            let x = Characteristic::name().as_str().to_owned();
            hm.insert(object_path, vec![(x, properties)].into_iter().collect());
            tracing::info!(
                "Characteristic {} registered with bluez",
                &object_path_clone
            );
        }

        let app = Application { objects: hm };
        let app_object_path = gatt_application
            .clone()
            .app_object_path
            .to_owned_object_path();
        object_server.at(&app_object_path, app).await?;
        let app_op = gatt_application.app_object_path.clone();
        tracing::info!("Application {app_op} added to object server");

        let gatt_manager_proxy = self.gatt_manager_proxy().await?;
        tracing::info!("gatt manager proxy created");
        gatt_manager_proxy
            .register_application(&app_object_path, HashMap::new())
            .await
            .inspect_err(|error| tracing::error!("Error: {}", error))?;
        tracing::info!("Application {app_op} registered with bluez");
        let gatt_manager_proxy = gatt_manager_proxy.clone();

        let application = gatt_application;

        Ok((
            rx,
            async move {
                gatt_manager_proxy
                    .unregister_application(&app_object_path)
                    .await?;
                tracing::info!("Application {app_op} unregistered with bluez");

                advertising_manager_proxy
                    .unregister_advertisement(&advertisement_path)
                    .await?;
                tracing::info!(
                    "Advertisement {} unregistered with bluez",
                    advertisement_path.as_str()
                );

                for characteristic in application.characteristics {
                    remove_from_server::<Characteristic>(
                        object_server,
                        &characteristic.object_path,
                    )
                    .await;
                }

                for service in application.services {
                    remove_from_server::<Service>(object_server, &service.object_path).await;
                }

                remove_from_server::<PeripheralAdvertisement>(object_server, &advertisement_path)
                    .await;
                remove_from_server::<Application>(object_server, &app_object_path).await;

                Ok::<(), zbus::fdo::Error>(())
            }
            .boxed(),
        ))
    }
}

#[cfg(test)]
mod tests {}
