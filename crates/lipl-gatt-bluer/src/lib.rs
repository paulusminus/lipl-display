use std::{
    collections::BTreeMap,
    sync::Arc, thread::JoinHandle, time::Duration,
};

use bluer::{
    adv::{
        Advertisement,
        AdvertisementHandle
    },
    gatt::local::{
            Application,
            ApplicationHandle,
            Characteristic,
            Service,
        },
    Uuid,
};
use lipl_display_common::{BackgroundThread, Message};

use futures::{channel::mpsc, Stream, StreamExt};
use tokio::sync::Mutex;
use log::{error, trace};
use pin_project::{pin_project, pinned_drop};
use std::pin::Pin;

mod error;
mod characteristic;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[pin_project(PinnedDrop)]
struct MessageStream {
    values_tx: futures::channel::mpsc::Sender<Message>,
    #[pin]
    values_rx: futures::channel::mpsc::Receiver<Message>,
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

impl futures::Stream for MessageStream {
    type Item = Message;
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
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

// async fn first_adapter() -> Result<bluer::Adapter> {
//     let session = bluer::Session::new().await?;
//     let adapter_names = session.adapter_names().await?;
//     let adapter_name = adapter_names.first().ok_or(lipl_display_common::Error::BluetoothAdapter)?;
//     let adapter: bluer::Adapter = session.adapter(adapter_name)?;
//     adapter.set_powered(true).await?;
//     Ok(adapter)
// }

async fn advertise(adapter: &bluer::Adapter) -> Result<AdvertisementHandle> {
    let mut manufacturer_data = BTreeMap::new();
    manufacturer_data.insert(lipl_display_common::MANUFACTURER_ID, vec![0x21, 0x22, 0x23, 0x24]);
    let le_advertisement = Advertisement {
        service_uuids: vec![lipl_display_common::SERVICE_UUID].into_iter().collect(),
        manufacturer_data,
        discoverable: Some(true),
        local_name: Some(lipl_display_common::LOCAL_NAME.to_owned()),
        ..Default::default()
    };
    let handle = adapter.advertise(le_advertisement).await?;
    Ok(handle)
    
}

pub struct ListenBluer {
    // callback: Box<dyn Fn(Message) + Send + 'static>,
    sender: Option<tokio::sync::oneshot::Sender<()>>,
    thread: Option<JoinHandle<()>>,
}

impl ListenBluer {
    pub fn new(callback: impl Fn(Message) + Send + 'static) -> Self {
        let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();
        let thread = std::thread::spawn(move || {
            let runtime = 
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Unable to create tokio runtime");
    
            runtime.block_on(async move {
                let mut s = 
                    listen_stream()
                        .await
                        .expect("Failed to start Gatt peripheral")
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
        ListenBluer { sender: Some(tx), thread: Some(thread) }
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
                            },
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

/// Start an extra thread that starts the gatt peripheral advertising included
// pub fn listen_background(cb: impl Fn(Message) -> Result<()> + Send + 'static) {
//     std::thread::spawn(move || {
//         let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().map_err(lipl_display_common::Error::IO)?;

//         runtime.block_on(async move {
//             let mut s = listen_stream().await?.boxed();
//             while let Some(message) = s.next().await {
//                 cb(message.clone())?;
//                 if message == Message::Command(Command::Exit) || message == Message::Command(Command::Poweroff) {
//                     break;
//                 }
//             }
//             Ok::<(), Error>(())
//         })
//     });
// }

/// Used in flutter version
pub async fn listen_stream() -> Result<impl Stream<Item=Message>> {
    let (values_tx, values_rx) = mpsc::channel::<Message>(100);

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    trace!("Bluetooth adapter {} found", adapter.name());

    let adv_handle = advertise(&adapter).await?;
    trace!("Advertising started");        
    let uuid: Uuid = lipl_display_common::SERVICE_UUID;
    let primary: bool = true;
    let characteristics: Vec<Characteristic> = 
        [
            lipl_display_common::CHARACTERISTIC_TEXT_UUID,
            lipl_display_common::CHARACTERISTIC_STATUS_UUID,
            lipl_display_common::CHARACTERISTIC_COMMAND_UUID
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

    Ok(
        MessageStream {
            values_tx,
            values_rx,
            adv_handle: Some(adv_handle),
            app_handle: Some(app_handle),       
        }
    )
}
