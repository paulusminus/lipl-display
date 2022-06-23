use std::{collections::HashMap, vec};

use futures_util::{
    StreamExt,
};
use tokio::{
    signal,
};
use uuid::{
    Uuid,
};
use zbus_bluez::{
    PeripheralConnection,
    GattApplicationConfig,
    GattApplicationConfigBuilder,
    GattCharacteristicConfigBuilder,
    GattServiceConfigBuilder,
    gatt::{Request, WriteRequest, ReadRequest},
};
use lipl_display_common::{
    Command,
    Message,
    CHARACTERISTIC_TEXT_UUID,
    CHARACTERISTIC_STATUS_UUID,
    CHARACTERISTIC_COMMAND_UUID,
    LOCAL_NAME,
    SERVICE_UUID
};

fn gatt_application_config() -> std::result::Result<GattApplicationConfig, Box<dyn std::error::Error>> {
    let char_text_config = 
        GattCharacteristicConfigBuilder::default()
        .uuid(CHARACTERISTIC_TEXT_UUID)
        .build()?;

    let char_status_config = 
        GattCharacteristicConfigBuilder::default()
        .uuid(CHARACTERISTIC_STATUS_UUID)
        .build()?;

    let char_command_config = 
        GattCharacteristicConfigBuilder::default()
        .uuid(CHARACTERISTIC_COMMAND_UUID)
        .build()?;

    let service_config =
        GattServiceConfigBuilder::default()
        .uuid(SERVICE_UUID)
        .characteristics(vec![
            char_text_config,
            char_status_config,
            char_command_config,
        ])
        .build()?;

    let app_config = GattApplicationConfigBuilder::default()
    .local_name(LOCAL_NAME.into())
    .services(
        vec![
            service_config,
        ]
    )
    .build()?;

    Ok(app_config)
}

fn handle_write_request(write_request: &mut WriteRequest, map: &mut HashMap<(Uuid, Uuid), Vec<u8>>) -> Option<Message> {
    let uuid = write_request.uuid;
    let service_uuid = write_request.service_uuid;
    match write_request.offset {
        Some(offset) => {
            log::error!("Cannot handle write request for {uuid} with offset {offset}");
            None
        },
        None => {
            match std::str::from_utf8(&write_request.value) {
                Ok(s) => {
                    map.entry((service_uuid, uuid)).and_modify(|e| *e = write_request.value.clone());
                    Message::try_from((s, uuid)).ok()
                }
                Err(_) => None,
            }    
        }
    }
}

fn handle_read_request(read_request: &mut ReadRequest, map: &HashMap<(Uuid, Uuid), Vec<u8>>) {
    if read_request.offset.is_none() {
        let uuid = read_request.uuid;
        let service_uuid = read_request.uuid;
        match read_request.sender.take() {
            Some(sender) => {
                let data = map[&(service_uuid, uuid)].clone();
                if let Err(error) = sender.send(data) {
                    log::error!("Error answering read request: {:?}", error); 
                }
            },
            None => {
                log::error!("Read request without channel");
            }
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> zbus::Result<()> {
    env_logger::init();

    let bluez = PeripheralConnection::new().await?;

    let (mut rx, dispose) = bluez.run(gatt_application_config().unwrap()).await?;
    log::info!("Advertising and Gatt application started");

    log::info!("Press <Ctr-C> or send signal SIGINT to end service");

    let mut map: HashMap<(Uuid, Uuid), Vec<u8>> = HashMap::new();
    map.insert((SERVICE_UUID, CHARACTERISTIC_TEXT_UUID), vec![]);
    map.insert((SERVICE_UUID, CHARACTERISTIC_STATUS_UUID), vec![]);
    map.insert((SERVICE_UUID, CHARACTERISTIC_COMMAND_UUID), vec![]);

    loop {
        tokio::select! {
            Some(mut request) = rx.next() => {
                match &mut request {
                    Request::Write(write_request) => {
                        if let Some(message) = handle_write_request(write_request, &mut map) {
                            log::info!("{:?}", message);
                            if message == Message::Command(Command::Poweroff) || message == Message::Command(Command::Exit) {
                                break;
                            }
                        };
                    },
                    Request::Read(read_request) => {
                        handle_read_request(read_request, &map);                           
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
