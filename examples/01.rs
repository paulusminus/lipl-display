use std::{collections::HashMap, vec};

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
    gatt::Request,
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

    let mut map: HashMap<Uuid, Vec<u8>> = HashMap::new();
    map.insert(CHARACTERISTIC_TEXT_UUID, vec![]);
    map.insert(CHARACTERISTIC_STATUS_UUID, vec![]);
    map.insert(CHARACTERISTIC_COMMAND_UUID, vec![]);

    loop {
        tokio::select! {
            Some(mut request) = rx.next() => {
                match &mut request {
                    Request::Write(write_request) => {
                        if write_request.offset.is_none() {
                            if let Ok(s) = std::str::from_utf8(&write_request.value) {
                                let uuid = write_request.uuid;
                                map.entry(uuid).and_modify(|e| *e = write_request.value.clone());
                                log::info!("Received value '{s}' for {uuid}");
                            }
                        }
                    },
                    Request::Read(read_request) => {
                        log::info!("Read requested");
                        if read_request.offset.is_none() {
                            let uuid = read_request.uuid;
                            let data = map[&uuid].clone();
                            if let Some(sender) = read_request.sender.take() {
                                match sender.send(data) {
                                    Ok(_) => { log::info!("Read request answered")},
                                    Err(error) => { log::error!("Error answering read request: {:?}", error);}
                                }
                            }
                        }
                        
                    },
                }
            },
            _ = signal::ctrl_c() => {
                break;
            }
        }
    }

    dispose().await?;

    Ok(())
}
