use std::sync::Arc;
use tokio::sync::Mutex;
use bluer::gatt::local::{Characteristic, CharacteristicWrite, CharacteristicWriteMethod, ReqError};
use futures::{FutureExt, channel::mpsc};
use futures::SinkExt;
use bluer::Uuid;
use crate::message::{Command, Message};
use crate::constant::{CHARACTERISTIC_COMMAND_UUID, CHARACTERISTIC_STATUS_UUID, CHARACTERISTIC_TEXT_UUID};

pub fn write_no_response_characteristic(uuid: Uuid, value_write: Arc<Mutex<Vec<u8>>>, sender: mpsc::Sender<crate::message::Message>) -> Characteristic {
    Characteristic {
        uuid,
        write: Some(CharacteristicWrite {
            write: true,
            write_without_response: true,
            method: CharacteristicWriteMethod::Fun(Box::new(move |new_value, _| {
                let value = value_write.clone();
                let mut s = sender.clone();
                async move {
                    let mut value = value.lock().await;
                    let send_value: Vec<u8> = new_value.to_vec();
                    *value = new_value;

                    if let Ok(received) = std::str::from_utf8(&send_value) {
                        if uuid == CHARACTERISTIC_TEXT_UUID.parse::<Uuid>().unwrap() {
                            s.send(Message::Part(received.to_owned())).await.map_err(|_| ReqError::Failed)?;
                        }
                        if uuid == CHARACTERISTIC_STATUS_UUID.parse::<Uuid>().unwrap() {
                            s.send(Message::Status(received.to_owned())).await.map_err(|_| ReqError::Failed)?;
                        }
                        if uuid == CHARACTERISTIC_COMMAND_UUID.parse::<Uuid>().unwrap() {
                            let command = received.parse::<Command>().map_err(|_| ReqError::Failed)?;
                            s.send(Message::Command(command)).await.map_err(|_| ReqError::Failed)?;
                        }
                    }
                    Ok(())
                }
                .boxed()
            })),
            ..Default::default()
        }),
        ..Default::default()
    }
}
