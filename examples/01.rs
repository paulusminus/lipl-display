use futures_util::{
    StreamExt,
};
use tokio::{
    signal,
};
use uuid::{
    uuid,
    Uuid,
};
use zbus_bluez::{
    PeripheralConnection,
    GattApplicationConfig,
    GattCharacteristicConfig,
    GattServiceConfig,
};

pub const SERVICE_UUID: Uuid = uuid!("27a70fc8-dc38-40c7-80bc-359462e4b808");
pub const LOCAL_NAME: &str = "lipl";

pub const CHARACTERISTIC_TEXT_UUID: Uuid = uuid!("04973569-c039-4ce9-ad96-861589a74f9e");
pub const CHARACTERISTIC_STATUS_UUID: Uuid = uuid!("61a8cb7f-d4c1-49b7-a3cf-f2c69dbb7aeb");
pub const CHARACTERISTIC_COMMAND_UUID: Uuid = uuid!("da35e0b2-7864-49e5-aa47-8050d1cc1484");


#[tokio::main(flavor = "current_thread")]
async fn main() -> zbus::Result<()> {
    env_logger::init();

    let gatt_application_config = GattApplicationConfig {
        app_object_path: "/org/bluez/app".into(),
        local_name: LOCAL_NAME.into(),
        services: vec![
            GattServiceConfig {
                primary: true,
                uuid: SERVICE_UUID,
                characteristics: vec![
                    GattCharacteristicConfig {
                        uuid: CHARACTERISTIC_TEXT_UUID,
                    },
                    GattCharacteristicConfig {
                        uuid: CHARACTERISTIC_STATUS_UUID,
                    },
                    GattCharacteristicConfig {
                        uuid: CHARACTERISTIC_COMMAND_UUID,
                    },
                ],
            },
        ],
    };

    let bluez = PeripheralConnection::new().await?;

    let (mut rx, dispose) = bluez.run(gatt_application_config).await?;
    log::info!("Advertising and Gatt application started");

    log::info!("Press <Ctr-C> or send signal SIGINT to end service");

    loop {
        tokio::select! {
            Some((uuid, s)) = rx.next() => {
                    log::info!("Received value ´{s}' from characteristic with uuid {uuid}");
            },
            _ = signal::ctrl_c() => {
                break;
            }
        }
    }

    dispose().await?;

    Ok(())
}
