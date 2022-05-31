use std::collections::HashMap;

use zbus::export::futures_util::TryFutureExt;
use zbus::zvariant::OwnedObjectPath;
use zbus_bluez::{BluezDbusConnection, advertisement::Advertisement, gatt_capable, Interfaces};

fn print_adapter(adapter: (&OwnedObjectPath, &Interfaces)) {
    println!("{}", adapter.0.as_str());
    adapter.1.keys().for_each(|s| { println!("  - {s}"); });
    println!();
}

fn print_adapters(adapters: HashMap<OwnedObjectPath, Interfaces>) {
    adapters.iter().for_each(print_adapter);
}

fn create_advertisement() -> (OwnedObjectPath, Advertisement) {
    (
        "/org/bluez/advertisement".try_into().unwrap(),
        Advertisement {
            advertisement_type: "peripheral".into(),
            manufacturer_data: HashMap::new(),
            service_uuids: vec!["".into()],
            sollicit_uuids: vec![],
            name: "jaja".into(),
            include_tx_power: true,
        }
    )
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> zbus::Result<()> {
    BluezDbusConnection::new(create_advertisement())
    .and_then(|bluez| async move { bluez.list_adapters(gatt_capable).await })
    .map_ok(print_adapters)
    .await?;

    println!("Press <Enter> to stop advertising");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(())
}
