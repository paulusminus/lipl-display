use std::{collections::HashMap, vec};
use std::convert::TryFrom;

use uuid::Uuid;
use crate::{
    GattApplicationConfig,
    GattApplicationConfigBuilder,
    GattCharacteristicConfigBuilder,
    GattServiceConfigBuilder,
    gatt::WriteRequest,
};
use lipl_display_common::{
    Message,
    CHARACTERISTIC_TEXT_UUID,
    CHARACTERISTIC_STATUS_UUID,
    CHARACTERISTIC_COMMAND_UUID,
    LOCAL_NAME,
    SERVICE_UUID
};

pub fn gatt_application_config() -> std::result::Result<GattApplicationConfig, Box<dyn std::error::Error>> {
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

pub fn handle_write_request(write_request: &mut WriteRequest, map: &mut HashMap<(Uuid, Uuid), Vec<u8>>) -> Option<Message> {
    let uuid = write_request.uuid;
    let service_uuid = write_request.service_uuid;
    match write_request.offset {
        Some(offset) => {
            tracing::error!("Cannot handle write request for {uuid} with offset {offset}");
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

pub fn characteristics_map() -> HashMap<(Uuid, Uuid), Vec<u8>> {
    let mut map: HashMap<(Uuid, Uuid), Vec<u8>> = HashMap::new();
    map.insert((SERVICE_UUID, CHARACTERISTIC_TEXT_UUID), vec![]);
    map.insert((SERVICE_UUID, CHARACTERISTIC_STATUS_UUID), vec![]);
    map.insert((SERVICE_UUID, CHARACTERISTIC_COMMAND_UUID), vec![]);
    map
}

