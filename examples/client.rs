use std::collections::HashMap;
use std::time::Duration;

use device::Device1Proxy;
use futures_util::StreamExt;
use lipl_display_common::{SERVICE_UUID, CHARACTERISTIC_TEXT_UUID, CHARACTERISTIC_STATUS_UUID, CHARACTERISTIC_COMMAND_UUID};
use zbus::{zvariant::{OwnedObjectPath, OwnedValue}, names::OwnedInterfaceName};
use zbus_bluez::PeripheralConnection;

const LIPL: &str = "43:45:C0:00:1F:AC";

type Objects = HashMap<OwnedObjectPath, HashMap<OwnedInterfaceName, HashMap<String, OwnedValue>>>;

fn get_object_paths(objects: &Objects, interface_name: &str) -> HashMap<OwnedObjectPath, HashMap<String, OwnedValue>> {
    objects
        .into_iter()
        .filter(|item| item.1.contains_key(interface_name))
        .map(|item| (item.0.clone(), item.1[interface_name].clone()))
        .collect()
}

fn get_device(objects: &Objects, address: &str) -> Option<OwnedObjectPath >{
    get_object_paths(objects, "org.bluez.Device1").into_iter().filter(|item| {
        let device_address: String = item.1["Address"].clone().try_into().unwrap();
        address.to_string() == device_address
    })
    .map(|item| item.0)
    .last()
}

fn get_characteristic_paths(objects: &Objects, device_address: &str) -> HashMap<String, String> {
    let mut characteristic_paths: HashMap<String, String> = HashMap::new();

    for (key, value) in get_object_paths(&objects, "org.bluez.Device1").iter() {
        let device = key.to_string();
        let address: String = value["Address"].clone().try_into().unwrap();

        if address == device_address.to_owned() {
            log::trace!("Device: {device}");

            for (key, value) in get_object_paths(&objects, "org.bluez.GattService1").iter() {
                let service = key.to_string();
                let service_device: String = OwnedObjectPath::try_from(value["Device"].clone()).unwrap().to_string();
                let uuid: String = value["UUID"].clone().try_into().unwrap();

                if service_device == device && uuid == SERVICE_UUID.to_string() {
                    log::trace!("  - Service: {uuid}");

                    for (key, value) in get_object_paths(&objects, "org.bluez.GattCharacteristic1").iter() {
                        let characteristic = key.to_string();
                        let uuid: String = value["UUID"].clone().try_into().unwrap();
                        let service_object_path = OwnedObjectPath::try_from(value["Service"].clone()).unwrap().to_string();
                        if service_object_path == service {
                            log::trace!("    - Characteristic: {uuid}");
                            characteristic_paths.insert(uuid, characteristic);
                        }
                    }
                }
            }
        }
    }
    characteristic_paths
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> zbus::Result<()> {
    env_logger::init();

    let bluez = PeripheralConnection::new().await?;
    let om = bluez.object_manager().await?;

    let objects = om
        .get_managed_objects().await?;

    if let Some(device_object_path) = get_device(&objects, LIPL) {
        let device_proxy = Device1Proxy::builder(bluez.connection()).destination("org.bluez")?.path(device_object_path)?.build().await?;
        let connected = device_proxy.connected().await?;
        if !connected {
            log::info!("Connecting to {}", LIPL);
            device_proxy.connect().await?;
            log::info!("Services resolved");
            let mut stream = device_proxy.receive_services_resolved_changed().await.boxed();
            while let Some(event) = stream.next().await {
                if event.get().await.unwrap() {
                    break;
                }
            }
            tokio::time::sleep(Duration::from_secs(3)).await;
        }

        for (uuid, path) in get_characteristic_paths(&objects, LIPL).iter() {
            if uuid == &CHARACTERISTIC_TEXT_UUID.to_string().to_lowercase() {
                log::info!("Text Uuid Path: {path}");
            }
    
            if uuid == &CHARACTERISTIC_STATUS_UUID.to_string().to_lowercase() {
                log::info!("Status Uuid Path: {path}");
            }
    
            if uuid == &CHARACTERISTIC_COMMAND_UUID.to_string().to_lowercase() {
                log::info!("Command Uuid Path: {path}");
            }
        }
    }

    Ok(())
}

#[allow(non_snake_case)]
mod device {
    use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.bluez.Device1")]
trait Device1 {
    /// CancelPairing method
    fn cancel_pairing(&self) -> zbus::Result<()>;

    /// Connect method
    fn connect(&self) -> zbus::Result<()>;

    /// ConnectProfile method
    fn connect_profile(&self, UUID: &str) -> zbus::Result<()>;

    /// Disconnect method
    fn disconnect(&self) -> zbus::Result<()>;

    /// DisconnectProfile method
    fn disconnect_profile(&self, UUID: &str) -> zbus::Result<()>;

    /// Pair method
    fn pair(&self) -> zbus::Result<()>;

    /// Adapter property
    #[dbus_proxy(property)]
    fn adapter(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Address property
    #[dbus_proxy(property)]
    fn address(&self) -> zbus::Result<String>;

    /// AddressType property
    #[dbus_proxy(property)]
    fn address_type(&self) -> zbus::Result<String>;

    /// Alias property
    #[dbus_proxy(property)]
    fn alias(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_alias(&self, value: &str) -> zbus::Result<()>;

    /// Appearance property
    #[dbus_proxy(property)]
    fn appearance(&self) -> zbus::Result<u16>;

    /// Blocked property
    #[dbus_proxy(property)]
    fn blocked(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_blocked(&self, value: bool) -> zbus::Result<()>;

    /// Class property
    #[dbus_proxy(property)]
    fn class(&self) -> zbus::Result<u32>;

    /// Connected property
    #[dbus_proxy(property)]
    fn connected(&self) -> zbus::Result<bool>;

    /// Icon property
    #[dbus_proxy(property)]
    fn icon(&self) -> zbus::Result<String>;

    /// LegacyPairing property
    #[dbus_proxy(property)]
    fn legacy_pairing(&self) -> zbus::Result<bool>;

    /// ManufacturerData property
    #[dbus_proxy(property)]
    fn manufacturer_data(
        &self,
    ) -> zbus::Result<std::collections::HashMap<u16, zbus::zvariant::OwnedValue>>;

    /// Modalias property
    #[dbus_proxy(property)]
    fn modalias(&self) -> zbus::Result<String>;

    /// Name property
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::Result<String>;

    /// Paired property
    #[dbus_proxy(property)]
    fn paired(&self) -> zbus::Result<bool>;

    /// RSSI property
    #[dbus_proxy(property)]
    fn rssi(&self) -> zbus::Result<i16>;

    /// ServiceData property
    #[dbus_proxy(property)]
    fn service_data(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// ServicesResolved property
    #[dbus_proxy(property)]
    fn services_resolved(&self) -> zbus::Result<bool>;

    /// Trusted property
    #[dbus_proxy(property)]
    fn trusted(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_trusted(&self, value: bool) -> zbus::Result<()>;

    /// TxPower property
    #[dbus_proxy(property)]
    fn tx_power(&self) -> zbus::Result<i16>;

    /// UUIDs property
    #[dbus_proxy(property)]
    fn uuids(&self) -> zbus::Result<Vec<String>>;

    /// WakeAllowed property
    #[dbus_proxy(property)]
    fn wake_allowed(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_wake_allowed(&self, value: bool) -> zbus::Result<()>;
}

#[dbus_proxy(interface = "org.bluez.MediaControl1")]
trait MediaControl1 {
    /// FastForward method
    fn fast_forward(&self) -> zbus::Result<()>;

    /// Next method
    fn next(&self) -> zbus::Result<()>;

    /// Pause method
    fn pause(&self) -> zbus::Result<()>;

    /// Play method
    fn play(&self) -> zbus::Result<()>;

    /// Previous method
    fn previous(&self) -> zbus::Result<()>;

    /// Rewind method
    fn rewind(&self) -> zbus::Result<()>;

    /// Stop method
    fn stop(&self) -> zbus::Result<()>;

    /// VolumeDown method
    fn volume_down(&self) -> zbus::Result<()>;

    /// VolumeUp method
    fn volume_up(&self) -> zbus::Result<()>;

    /// Connected property
    #[dbus_proxy(property)]
    fn connected(&self) -> zbus::Result<bool>;

    /// Player property
    #[dbus_proxy(property)]
    fn player(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}

}