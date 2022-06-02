use std::vec;

use tokio::signal;
use zbus_bluez::bluez_interfaces::Adapter1Proxy;
use zbus_bluez::{BluezDbusConnection};
use zbus_bluez::advertisement::PeripheralAdvertisement;

async fn print_adapter(adapter: Adapter1Proxy<'_>) -> zbus::Result<()> {
    let name = adapter.name().await?;
    let address = adapter.address().await?;
    let path = adapter.path();
    log::info!("{} ({}) on path {} powered on and discoverable", name, address, path);

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> zbus::Result<()> {
    env_logger::init();

    let bluez = BluezDbusConnection::new().await?;
    let adapter = bluez.adapter_proxy().await?;
    print_adapter(adapter).await?;
    let advertisement = PeripheralAdvertisement::new(
        "lipl".into(),
        vec![zbus_bluez::SERVICE_1_UUID],
    );

    let unregister_advertisement = bluez.register_advertisement(advertisement).await?;
    log::info!("Advertising started");

    let _app = bluez.register_application().await?;

    log::info!("Press <Ctr-C> to quit service");
    signal::ctrl_c().await?;

    unregister_advertisement().await?;

    Ok(())
}
