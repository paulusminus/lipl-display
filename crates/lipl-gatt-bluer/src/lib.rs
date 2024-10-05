#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use std::{collections::BTreeMap, sync::Arc, thread::JoinHandle, time::Duration};

use bluer::{
    adv::{Advertisement, AdvertisementHandle},
    gatt::local::{Application, ApplicationHandle, Characteristic, Service},
    Uuid,
};
use lipl_display_common::{BackgroundThread, Message};

use futures_channel::mpsc;
use futures_util::{Stream, StreamExt};
use log::{error, trace};
use pin_project::{pin_project, pinned_drop};
use std::pin::Pin;
use tokio::sync::Mutex;

mod characteristic;
mod error;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[pin_project(PinnedDrop)]
pub struct MessageStream {
    values_tx: mpsc::Sender<Message>,
    #[pin]
    values_rx: mpsc::Receiver<Message>,
    adv_handle: Option<AdvertisementHandle>,
    app_handle: Option<ApplicationHandle>,
}

#[pinned_drop]
impl PinnedDrop for MessageStream {
    fn drop(self: Pin<&mut Self>) {
        let this = self.project();
        if let Some(handle) = this.adv_handle.take() {
            drop(handle);
            trace!("Handle dropped for Advertisement");
        };
        if let Some(handle) = this.app_handle.take() {
            drop(handle);
            trace!("Handle dropped for Application");
        };
    }
}

impl Stream for MessageStream {
    type Item = Message;
    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().values_rx.poll_next(cx)
    }
}

/// Utility function so that dependent crates do not need tokio dependency
pub fn create_runtime() -> Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| lipl_display_common::Error::Runtime)
        .map_err(Error::Common)
}

async fn advertise(adapter: &bluer::Adapter) -> Result<AdvertisementHandle> {
    let mut manufacturer_data = BTreeMap::new();
    manufacturer_data.insert(
        lipl_display_common::MANUFACTURER_ID,
        vec![0x21, 0x22, 0x23, 0x24],
    );
    let le_advertisement = Advertisement {
        service_uuids: vec![lipl_display_common::SERVICE_UUID]
            .into_iter()
            .collect(),
        manufacturer_data,
        discoverable: Some(true),
        local_name: Some(lipl_display_common::LOCAL_NAME.to_owned()),
        tx_power: Some(8),
        ..Default::default()
    };
    let handle = adapter.advertise(le_advertisement).await?;
    Ok(handle)
}

pub struct ListenBluer {
    sender: Option<tokio::sync::oneshot::Sender<()>>,
    thread: Option<JoinHandle<()>>,
}

fn wait() -> impl Stream<Item = Message> {
    futures_util::stream::once(async { Message::Command(lipl_display_common::Command::Wait) })
}

impl ListenBluer {
    pub fn new(callback: impl Fn(Message) + Send + 'static) -> Self {
        let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();
        let thread = std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Unable to create tokio runtime");

            runtime.block_on(async move {
                let mut s = wait().chain(
                        listen_stream()
                        .await
                        .expect("Failed to start Gatt peripheral")    
                    )
                    .boxed();
                loop {
                    tokio::select! {
                        option_message = s.next() => {
                            match option_message {
                                Some(message) => {
                                    callback(message.clone());
                                }
                                None => break,
                            }
                        }
                        received = &mut rx => {
                            match received {
                                Ok(_) => {
                                    break;
                                },
                                Err(error) => {
                                    log::error!("Error receiving signal to quit background thread: {}", error);
                                    break;
                                },
                            }
                        }
                    }
                }
            });
            log::info!("Background thread almost finished");
        });
        ListenBluer {
            sender: Some(tx),
            thread: Some(thread),
        }
    }
}

impl BackgroundThread for ListenBluer {
    fn stop(&mut self) {
        if let Some(tx) = self.sender.take() {
            match tx.send(()) {
                Ok(_) => {
                    if let Some(thread) = self.thread.take() {
                        match thread.join() {
                            Ok(_) => {
                                std::thread::sleep(Duration::from_secs(1));
                                trace!("Finished sleeping for 1 second");
                            }
                            Err(_) => {
                                error!("Error joining background thread");
                            }
                        }
                    }
                }
                Err(_) => {
                    error!("Error sending signal to background thread");
                }
            }
        }
    }
}

/// Used in flutter version
pub async fn listen_stream() -> Result<MessageStream> {
    let (values_tx, values_rx) = mpsc::channel::<Message>(100);

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    trace!("Bluetooth adapter {} found", adapter.name());
    let capabilities = adapter.supported_advertising_capabilities().await?;
    if let Some(caps) = capabilities {
        trace!(
            "max advertisement length: {}",
            caps.max_advertisement_length
        );
        trace!(
            "max scan reponse length : {}",
            caps.max_scan_response_length
        );
        trace!("max tx power: {}", caps.max_tx_power);
        trace!("min tx power: {}", caps.min_tx_power);
    }

    let adv_handle = advertise(&adapter).await?;
    trace!("Advertising started");
    let uuid: Uuid = lipl_display_common::SERVICE_UUID;
    let primary: bool = true;
    let characteristics: Vec<Characteristic> = [
        lipl_display_common::CHARACTERISTIC_TEXT_UUID,
        lipl_display_common::CHARACTERISTIC_STATUS_UUID,
        lipl_display_common::CHARACTERISTIC_COMMAND_UUID,
    ]
    .into_iter()
    .map(|c| (c, Arc::new(Mutex::new(vec![]))))
    .map(|v| characteristic::write_no_response_characteristic(v.0, v.1, values_tx.clone()))
    .collect();

    let app = Application {
        services: vec![Service {
            uuid,
            primary,
            characteristics,
            ..Default::default()
        }],
        ..Default::default()
    };

    let app_handle = adapter.serve_gatt_application(app).await?;

    Ok(MessageStream {
        values_tx,
        values_rx,
        adv_handle: Some(adv_handle),
        app_handle: Some(app_handle),
    })
}
