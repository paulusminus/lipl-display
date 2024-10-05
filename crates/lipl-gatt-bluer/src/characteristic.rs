use bluer::gatt::local::{
    Characteristic, CharacteristicWrite, CharacteristicWriteMethod, ReqError,
};
use bluer::Uuid;
use futures_channel::mpsc;
use futures_util::{FutureExt, SinkExt};
use lipl_display_common::Message;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn write_no_response_characteristic(
    uuid: Uuid,
    value_write: Arc<Mutex<Vec<u8>>>,
    sender: mpsc::Sender<Message>,
) -> Characteristic {
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
                        let message =
                            Message::try_from((received, uuid)).map_err(|_| ReqError::Failed)?;
                        s.send(message).await.map_err(|_| ReqError::Failed)?;
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
