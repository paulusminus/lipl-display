use std::{collections::HashMap};

use zbus::{export::futures_util::TryFutureExt};
use zbus::zvariant::OwnedObjectPath;
use zbus_bluez::{BluezDbusConnection, advertisement::{Advertisement, SERVICE_UUID}, gatt_capable, Interfaces};

fn print_adapter(adapter: (&OwnedObjectPath, &Interfaces)) {
    println!("{}", adapter.0.as_str());
    adapter.1.keys().for_each(|s| { println!("  - {s}"); });
    println!();
}

fn print_adapters(adapters: HashMap<OwnedObjectPath, Interfaces>) {
    adapters.iter().for_each(print_adapter);
}

// fn manufacturer_data() -> HashMap<u16, Vec<u8>> {
//     let mut hm = HashMap::new();
//     hm.insert(0xFF, vec![0x45]);
//     hm
// }

fn create_advertisement() -> (OwnedObjectPath, Advertisement) {
    (
        "/org/bluez/advertisement".try_into().unwrap(),
        Advertisement {
            advertisement_type: "peripheral".into(),
            // manufacturer_data: manufacturer_data(),
            service_uuids: vec![SERVICE_UUID.into()],
            local_name: "lipl-zbus".into(),
            include_tx_power: true,
        }
    )
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> zbus::Result<()> {
    env_logger::init();
    BluezDbusConnection::new(create_advertisement())
    .and_then(|bluez| async move { bluez.list_adapters(gatt_capable).await })
    .map_ok(print_adapters)
    .await?;

    println!("Press <Enter> to stop advertising");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(())
}
