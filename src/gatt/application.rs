use std::{collections::HashMap};

use uuid::{uuid, Uuid};
use zbus::{Connection, dbus_interface, zvariant::{OwnedObjectPath, OwnedValue}, fdo::PropertiesProxy, names::InterfaceName};
use super::{Service, Characteristic};

pub struct Application {
    objects: HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>>,
}

#[dbus_interface(name = "org.freedesktop.DBus.ObjectManager")]
impl Application {
    fn get_managed_object(&self) -> HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>> {
        self.objects.clone()
    }
}

async fn create_application(connection: Connection) -> zbus::Result<()>{
    let create_path = |uuid: Uuid| format!("/org/bluez/app/lipl/{}", uuid);
    let mut hm: HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>> = HashMap::new();

    let service1_uuid = uuid!("b7423a7c-08cc-483d-bdf6-d351a7656a70");
    let char1_uuid = uuid!("d062f9a4-f41d-4b80-93af-0bf2a32c26bf");
    let char2_uuid = uuid!("935d864b-61fe-4c55-8b9e-53be40919950");

    let char1 = Characteristic {
        uuid: char1_uuid,
        read: false,
        write: true,
        notify: false,
        service_path: create_path(service1_uuid),
        descriptor_paths: vec![],
        value: String::new(),
    };
    connection.object_server().at(create_path(char1_uuid), char1).await?;
    let proxy = PropertiesProxy::builder(&connection).path(create_path(char1_uuid))?.build().await?;
    let char1_props = proxy.get_all(InterfaceName::try_from("org.bluez.GattCharacteristic1").unwrap()).await?;
    hm.insert(
        OwnedObjectPath::try_from(create_path(char1_uuid)).unwrap(),
        vec![("org.bluez.GattCharacteristic1".to_owned(), char1_props)].into_iter().collect()
    );

    let char2 = Characteristic {
        uuid: char2_uuid,
        read: false,
        write: true,
        notify: false,
        service_path: create_path(service1_uuid),
        descriptor_paths: vec![],
        value: String::new(),
    };
    connection.object_server().at(create_path(char2_uuid), char2).await?;
    let proxy = PropertiesProxy::builder(&connection).path(create_path(char2_uuid))?.build().await?;
    let char2_props = proxy.get_all(InterfaceName::try_from("org.bluez.GattCharacteristic1").unwrap()).await?;
    hm.insert(
        OwnedObjectPath::try_from(create_path(char1_uuid)).unwrap(),
        vec![("org.bluez.GattCharacteristic1".to_owned(), char2_props)].into_iter().collect(),
    );

    let service1 = Service {
        primary: true,
        uuid: service1_uuid,
        characteristic_paths: vec![
            create_path(char1_uuid),
            create_path(char2_uuid),
        ]
    };
    connection.object_server().at(create_path(service1_uuid), service1).await?;
    let proxy = PropertiesProxy::builder(&connection).path(create_path(service1_uuid))?.build().await?;
    let service1_props = proxy.get_all(InterfaceName::try_from("org.bluez.GattService1").unwrap()).await?;
    hm.insert(
        OwnedObjectPath::try_from(create_path(service1_uuid)).unwrap(),
        vec![("org.bluez.GattService1".to_owned(), service1_props)].into_iter().collect()
    );

    let app1_uuid = uuid!("db4b8967-d115-4393-8de4-23f4c1ea6d5c");
    let app = Application {
        objects: hm,
    };
    connection.object_server().at(create_path(app1_uuid), app).await?;

    Ok(())
}