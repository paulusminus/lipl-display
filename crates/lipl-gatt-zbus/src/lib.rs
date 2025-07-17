pub use error::Result;
use futures::{
    FutureExt, SinkExt, Stream, StreamExt, TryFutureExt,
    channel::mpsc::{Receiver, Sender},
    select,
};
use gatt::Request;
use gatt_application::GattCharacteristicConfig;
use lipl_display_common::{Command, Message};
use message_handler::{characteristics_map, handle_write_request};
use peripheral::Peripheral;
use pin_project::pin_project;
use std::collections::HashMap;
use std::pin::Pin;
use std::task::{Context, Poll};
use zbus::{names::OwnedInterfaceName, zvariant::OwnedValue};

mod advertisement;
mod connection_extension;
mod error;
mod gatt;
mod gatt_application;
mod message_handler;
mod object_path_extensions;
mod peripheral;
mod proxy;

type Interfaces = HashMap<OwnedInterfaceName, HashMap<String, OwnedValue>>;

#[pin_project]
pub struct GattListener {
    task: tokio::task::JoinHandle<()>,
    #[pin]
    receiver: futures::channel::mpsc::Receiver<Message>,
    terminate: futures::channel::oneshot::Sender<()>,
}

impl Stream for GattListener {
    type Item = Message;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().receiver.poll_next(cx)
    }
}

impl Default for GattListener {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoFuture for GattListener {
    type Output = Result<()>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'static>>;

    fn into_future(self) -> Self::IntoFuture {
        if !self.task.is_finished() {
            self.terminate.send(()).ok();
        }
        self.task.map_err(Into::into).boxed()
    }
}

impl GattListener {
    pub fn new() -> Self {
        let (sender, receiver) = futures::channel::mpsc::channel::<Message>(100);
        let (terminate, terminate_receiver) = futures::channel::oneshot::channel::<()>();
        Self {
            task: tokio::runtime::Handle::current().spawn(async move {
                match Peripheral::new()
                    .and_then(|bluez| {
                        bluez.run(message_handler::gatt_application_config().unwrap())
                    })
                    .await
                {
                    Ok((rx, dispose)) => {
                        handle_messages(rx, sender, terminate_receiver, dispose).await;
                    }
                    Err(error) => {
                        tracing::error!("Error initializing Bluetooth: {}", error);
                    }
                }
            }),
            receiver,
            terminate,
        }
    }
}

async fn handle_messages(
    mut rx: Receiver<Request>,
    mut sender: Sender<Message>,
    mut terminate_receiver: futures::channel::oneshot::Receiver<()>,
    dispose: Pin<Box<dyn Future<Output = Result<()>> + Send>>,
) {
    tracing::info!("Advertising and Gatt application started");
    tracing::info!("Press <Ctr-C> or send signal SIGINT to end service");

    let mut map = characteristics_map();

    loop {
        select! {
            request = rx.next() => {
                match request {
                    Some(Request::Write(mut write_request)) => {
                        if let Some(message) = handle_write_request(&mut write_request, &mut map) {
                            if [Message::Command(Command::Exit), Message::Command(Command::Poweroff)].contains(&message)
                            {
                                break;
                            }
                            sender.send(message).await.unwrap();
                        }
                    }
                    Some(Request::Read(_)) => {
                        todo!("Implement read request handling");
                    }
                    None => {
                        tracing::info!("No more requests");
                        break;
                    }
                }
            },
            _ = terminate_receiver => break,
        }
    }
    dispose.await.expect("Cannot dispose");
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    struct Characteristic {
        uuid: Uuid,
        read: bool,
        write: bool,
    }

    struct Service {
        uuid: Uuid,
        primary: bool,
        characteristics: &'static [Characteristic],
    }

    struct App {
        services: &'static [Service],
    }

    const LIPL_DISPLAY_SERVICE: Service = Service {
        uuid: lipl_display_common::SERVICE_UUID,
        primary: true,
        characteristics: &[
            Characteristic {
                uuid: lipl_display_common::CHARACTERISTIC_TEXT_UUID,
                read: false,
                write: true,
            },
            Characteristic {
                uuid: lipl_display_common::CHARACTERISTIC_STATUS_UUID,
                read: false,
                write: true,
            },
            Characteristic {
                uuid: lipl_display_common::CHARACTERISTIC_COMMAND_UUID,
                read: false,
                write: true,
            },
        ],
    };

    const APP: App = App {
        services: &[LIPL_DISPLAY_SERVICE],
    };

    #[test]
    fn test_app() {
        assert_eq!(APP.services.len(), 1);
        assert_eq!(APP.services[0].uuid, LIPL_DISPLAY_SERVICE.uuid);
        assert_eq!(APP.services[0].primary, true);
        assert_eq!(APP.services[0].characteristics.len(), 3);
    }

    #[test]
    fn test_service() {
        assert_eq!(LIPL_DISPLAY_SERVICE.uuid, lipl_display_common::SERVICE_UUID);
        assert_eq!(LIPL_DISPLAY_SERVICE.primary, true);
        assert_eq!(LIPL_DISPLAY_SERVICE.characteristics.len(), 3);
    }

    #[test]
    fn test_characteristic() {
        assert_eq!(
            LIPL_DISPLAY_SERVICE.characteristics[0].uuid,
            lipl_display_common::CHARACTERISTIC_TEXT_UUID
        );
        assert_eq!(LIPL_DISPLAY_SERVICE.characteristics[0].read, false);
        assert_eq!(LIPL_DISPLAY_SERVICE.characteristics[0].write, true);
        assert_eq!(
            LIPL_DISPLAY_SERVICE.characteristics[1].uuid,
            lipl_display_common::CHARACTERISTIC_STATUS_UUID
        );
        assert_eq!(LIPL_DISPLAY_SERVICE.characteristics[1].read, false);
        assert_eq!(LIPL_DISPLAY_SERVICE.characteristics[1].write, true);
        assert_eq!(
            LIPL_DISPLAY_SERVICE.characteristics[2].uuid,
            lipl_display_common::CHARACTERISTIC_COMMAND_UUID
        );
        assert_eq!(LIPL_DISPLAY_SERVICE.characteristics[2].read, false);
        assert_eq!(LIPL_DISPLAY_SERVICE.characteristics[2].write, true);
    }
}
