use crate::{
    Result,
    advertisement::PeripheralAdvertisement,
    connection_extension::ConnectionExt,
    error::Error,
    gatt::{Application, Characteristic, Request, Service},
    gatt_application::{GattApplication, GattApplicationConfig},
    object_path_extensions::OwnedObjectPathExtensions,
    proxy::{Adapter1Proxy, GattManager1Proxy, LEAdvertisingManager1Proxy},
};
use futures::{
    FutureExt,
    channel::mpsc::{Receiver, channel},
};
use std::{collections::HashMap, pin::Pin};
use zbus::{
    Connection, ObjectServer,
    conn::Builder,
    object_server::Interface,
    zvariant::{OwnedObjectPath, OwnedValue},
};

#[derive(Clone)]
pub(crate) struct Peripheral {
    connection: Connection,
    adapter: OwnedObjectPath,
}

impl Peripheral {
    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    pub fn object_server(&self) -> &ObjectServer {
        self.connection.object_server()
    }

    pub async fn adapter_proxy(&self) -> Result<Adapter1Proxy<'_>> {
        Adapter1Proxy::builder(&self.connection)
            .destination("org.bluez")?
            .path(&self.adapter)?
            .build()
            .await
            .map_err(Into::into)
    }

    pub async fn gatt_manager_proxy(&self) -> Result<GattManager1Proxy<'_>> {
        GattManager1Proxy::builder(&self.connection)
            .destination("org.bluez")?
            .path(&self.adapter)?
            .build()
            .await
            .map_err(Into::into)
    }

    pub async fn advertising_manager_proxy(&self) -> Result<LEAdvertisingManager1Proxy<'_>> {
        LEAdvertisingManager1Proxy::builder(&self.connection)
            .destination("org.bluez")?
            .path(&self.adapter)?
            .build()
            .await
            .map_err(Into::into)
    }

    /// Creates a dbus connection to bluez
    /// Finds the first gatt capable adapter
    /// Set adapter powered and discoverable1
    pub async fn new() -> Result<Peripheral> {
        let connection = Builder::system()?.build().await?;
        let adapter = connection.first_gatt_capable_adapter().await?;
        let peripheral_connection = Peripheral {
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
        self,
        gatt_application_config: GattApplicationConfig,
    ) -> Result<(
        Receiver<Request>,
        Pin<Box<(dyn Future<Output = Result<()>> + Send)>>,
    )> {
        let (tx, rx) = channel::<Request>(1);
        // let object_server = self.connection.object_server().clone();
        let gatt_application: GattApplication = (gatt_application_config, tx).into();

        // Advertising
        let advertisement = PeripheralAdvertisement::from(&gatt_application);
        let advertisement_path =
            format!("{}/advertisement", gatt_application.app_object_path).to_owned_object_path();
        let advertising_manager_proxy = self.advertising_manager_proxy().await?.clone();
        self.object_server()
            .at(&advertisement_path, advertisement)
            .await?;
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
            self.object_server()
                .at(&object_path, service.clone())
                .await?;
            let interface = self
                .object_server()
                .interface::<_, Service>(&object_path)
                .await?;
            let properties = service
                .get_all(
                    self.object_server(),
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
                .object_server()
                .interface::<_, Characteristic>(&object_path)
                .await?;
            let properties = characteristic
                .get_all(
                    self.object_server(),
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
        self.object_server().at(&app_object_path, app).await?;
        let app_op = gatt_application.app_object_path.clone();
        tracing::info!("Application {app_op} added to object server");

        let gatt_manager_proxy = self.gatt_manager_proxy().await?;
        tracing::info!("gatt manager proxy created");
        gatt_manager_proxy
            .register_application(&app_object_path, HashMap::new())
            .await
            .inspect_err(|error| tracing::error!("Error: {}", error))?;
        tracing::info!("Application {app_op} registered with bluez");

        let application = gatt_application;

        Ok((
            rx,
            async move {
                let gatt_manager_proxy = self.gatt_manager_proxy().await?;
                let advertising_manager_proxy = self.advertising_manager_proxy().await?;
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
                        self.object_server(),
                        &characteristic.object_path,
                    )
                    .await;
                }

                for service in application.services {
                    remove_from_server::<Service>(self.object_server(), &service.object_path).await;
                }

                remove_from_server::<PeripheralAdvertisement>(
                    self.object_server(),
                    &advertisement_path,
                )
                .await;
                remove_from_server::<Application>(self.object_server(), &app_object_path).await;

                Ok::<(), Error>(())
            }
            .boxed(),
        ))
    }
}

async fn remove_from_server<I: Interface>(object_server: &ObjectServer, object_path: &str) {
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
