use std::{collections::HashMap};

use uuid::{uuid, Uuid};
use zbus::{
    Connection,
    dbus_interface,
    zvariant::{
        OwnedObjectPath,
        OwnedValue
    },
    Interface,
};
use crate::bluez_interfaces::GattManager1Proxy;

use super::{Service, Characteristic};

const APP_1_PATH: &str = "/org/bluez/app1";
pub const SERVICE_1_UUID: Uuid = uuid!("b7423a7c-08cc-483d-bdf6-d351a7656a70");
const SERVICE_1_PATH: &str = "/org/bluez/app1/service1";
const CHAR_1_UUID: Uuid = uuid!("d062f9a4-f41d-4b80-93af-0bf2a32c26bf");
const CHAR_1_PATH: &str = "/org/bluez/app1/service1/char1";
const CHAR_2_UUID: Uuid = uuid!("935d864b-61fe-4c55-8b9e-53be40919950");
const CHAR_2_PATH: &str = "/org/bluez/app1/service1/char2";

const CHAR1: Characteristic = Characteristic {
    uuid: CHAR_1_UUID,
    read: false,
    write: true,
    notify: false,
    service_path: SERVICE_1_PATH,
    descriptor_paths: vec![],
    value: String::new(),
};

const CHAR2: Characteristic = Characteristic {
    uuid: CHAR_2_UUID,
    read: false,
    write: true,
    notify: false,
    service_path: SERVICE_1_PATH,
    descriptor_paths: vec![],
    value: String::new(),
};

const SERVICE1: Service = Service {
    primary: true,
    uuid: SERVICE_1_UUID,
    characteristic_paths: &[
        CHAR_1_PATH,
        CHAR_2_PATH,
    ]
};

#[derive(Clone, Debug)]
pub struct Application {
    objects: HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>>,
}

#[dbus_interface(name = "org.freedesktop.DBus.ObjectManager")]
impl Application {
    #[dbus_interface(name = "GetManagedObjects")]
    fn get_managed_objects(&self) -> HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>> {
        log::info!("Get managed objects called");
        log::info!("{:#?}", self.objects);
        self.objects.clone()
    }
}

pub async fn register_application(connection: &Connection) -> zbus::Result<Application>{
    let mut hm: HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>> = HashMap::new();
    log::info!("Whatever");

    connection.object_server().at(SERVICE_1_PATH, SERVICE1.clone()).await?;
    log::info!("Service 1 registered at {}", SERVICE_1_PATH);
    let service1_props = SERVICE1.get_all().await;
    hm.insert(
        OwnedObjectPath::try_from(SERVICE_1_PATH).unwrap(),
        vec![
            ("org.bluez.GattService1".to_owned(), service1_props)
        ]
        .into_iter()
        .collect()
    );

    connection.object_server().at(CHAR_1_PATH, CHAR1.clone()).await?;
    log::info!("Characteristic 1 registered at {}", CHAR_1_PATH);

    let char1_props = CHAR1.get_all().await;
    hm.insert(
        OwnedObjectPath::try_from(CHAR_1_PATH).unwrap(),
        vec![("org.bluez.GattCharacteristic1".to_owned(), char1_props)].into_iter().collect()
    );

    connection.object_server().at(CHAR_2_PATH, CHAR2.clone()).await?;
    log::info!("Characteristic 2 registered at {}", CHAR_2_PATH);
    let char2_props = CHAR2.get_all().await;
    hm.insert(
        OwnedObjectPath::try_from(CHAR_2_PATH).unwrap(),
        vec![("org.bluez.GattCharacteristic1".to_owned(), char2_props)].into_iter().collect(),
    );

    let app = Application {
        objects: hm,
    };
    connection.object_server().at(APP_1_PATH, app.clone()).await?;
    log::info!("Application 1 registered at {}", APP_1_PATH);

    let proxy = GattManager1Proxy::builder(connection).destination("org.bluez")?.path("/org/bluez/hci0")?.build().await?;
    proxy.register_application(
        &OwnedObjectPath::try_from(APP_1_PATH).unwrap(),
        HashMap::new()
    )
    .await?;
    log::info!("Application 1 registered with bluez");

    Ok(app)
}