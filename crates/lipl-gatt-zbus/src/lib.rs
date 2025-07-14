use futures::{FutureExt, Stream, StreamExt, TryFutureExt, select};
use gatt::Request;
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
mod gatt;
mod gatt_application;
mod message_handler;
mod object_path_extensions;
mod peripheral;
mod proxy;

use gatt_application::{
    GattApplicationConfig, GattApplicationConfigBuilder, GattCharacteristicConfig,
    GattCharacteristicConfigBuilder, GattServiceConfigBuilder,
};
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
    type Output = std::io::Result<()>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'static>>;

    fn into_future(self) -> Self::IntoFuture {
        self.terminate.send(()).ok();
        self.task.map_err(std::io::Error::other).boxed()
    }
}

impl GattListener {
    pub fn new() -> Self {
        let (mut sender, receiver) = futures::channel::mpsc::channel::<Message>(100);
        let (terminate, mut terminate_receiver) = futures::channel::oneshot::channel::<()>();
        Self {
            task: tokio::runtime::Handle::current().spawn(async move {
                match Peripheral::new()
                    .and_then(|bluez| bluez.run(message_handler::gatt_application_config().unwrap())).await {
                        Ok((mut rx, dispose)) => {
                            tracing::info!("Advertising and Gatt application started");

                            tracing::info!("Press <Ctr-C> or send signal SIGINT to end service");

                            let mut map = characteristics_map();

                            loop {
                                select! {
                                    request = rx.next() => {
                                        if let Some(Request::Write(mut write_request)) = request {
                                            if let Some(message) = handle_write_request(&mut write_request, &mut map) {
                                                sender.try_send(message.clone()).unwrap();
                                                if message == Message::Command(Command::Exit)
                                                    || message == Message::Command(Command::Poweroff)
                                                {
                                                    break;
                                                }
                                            }
                                        }
                                    },
                                    _ = terminate_receiver => break,
                                }
                            }
                            dispose.await.expect("Cannot dispose");
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

#[cfg(test)]
mod tests {}
