use std::vec;

use zbus_bluez::bluez_interfaces::Adapter1Proxy;
use zbus_bluez::{BluezDbusConnection};
use zbus_bluez::advertisement::PeripheralAdvertisement;
use uuid::{uuid, Uuid};

pub const SERVICE_UUID: Uuid = uuid!("27a70fc8-dc38-40c7-80bc-359462e4b808");

async fn print_adapter(adapter: Adapter1Proxy<'_>) -> zbus::Result<()> {
    let name = adapter.name().await?;
    let address = adapter.address().await?;
    println!("{} ({})", name, address);

    Ok(())
}


#[tokio::main(flavor = "current_thread")]
async fn main() -> zbus::Result<()> {
    env_logger::init();
    let bluez = BluezDbusConnection::new().await?;
    let adapter = bluez.adapter_proxy().await?;
    print_adapter(adapter).await?;
    let advertisement = PeripheralAdvertisement::new("fedora".into(), vec![SERVICE_UUID] );
    bluez.register_advertisement(advertisement).await?;

    println!("Press <Enter> to stop advertising");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(())
}
