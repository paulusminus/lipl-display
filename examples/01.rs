use std::vec;

use futures_util::{StreamExt};
use tokio::signal;
use uuid::uuid;
use zbus_bluez::{
    BluezDbusConnection,
    GattApplicationConfig,
    GattCharacteristicConfig,
    GattServiceConfig,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> zbus::Result<()> {
    env_logger::init();

    let gatt_application_config = GattApplicationConfig {
        app_object_path: "/org/bluez/app".into(),
        local_name: "lipl".into(),
        services: vec![
            GattServiceConfig {
                primary: true,
                uuid: uuid!("5117859b-f9b1-4e8c-bacf-a9d900237d3a"),
                characteristics: vec![
                    GattCharacteristicConfig {
                        uuid: uuid!("82000e45-a116-4ab6-a88c-a5b7f9d5e9e6"),
                    },
                    GattCharacteristicConfig {
                        uuid: uuid!("4460e7a6-4684-4657-9ad7-70a52595e196"),
                    }
                ]
            }
        ],
    };

    let bluez = BluezDbusConnection::new().await?;

    let (mut rx, dispose) = bluez.run(gatt_application_config).await?;
    log::info!("Advertising and Gatt application started");

    log::info!("Press <Ctr-C> or send signal SIGINT to end service");

    loop {
        tokio::select! {
            Some((uuid, s)) = rx.next() => {
                    log::info!("Received value {s} from characteristic with uuid {uuid}");
            },
            _ = signal::ctrl_c() => {
                break;
            }
        }
    }

    dispose().await?;

    Ok(())
}
