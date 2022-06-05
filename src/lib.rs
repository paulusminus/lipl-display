use std::{
    collections::{
        BTreeMap,
    },
    sync::{
        Arc,
    },
};

use bluer::{
    adv::{
        Advertisement,
        AdvertisementHandle
    },
    gatt::{
        local::{
            Application,
            ApplicationHandle,
            Characteristic,
            Service,
        },
    },
    Uuid,
};
pub use lipl_display_common::Command;
pub use lipl_display_common::Message;

use futures::{channel::mpsc, Stream, StreamExt};
use tokio::{sync::Mutex};
use log::{trace};
use pin_project::{pin_project, pinned_drop};
use std::pin::Pin;

mod error;
mod characteristic;

pub use error::{Error, Result};

#[pin_project(PinnedDrop)]
struct ValuesStream {
    values_tx: futures::channel::mpsc::Sender<Message>,
    #[pin]
    values_rx: futures::channel::mpsc::Receiver<Message>,
    adv_handle: Option<AdvertisementHandle>,
    app_handle: Option<ApplicationHandle>,
}

#[pinned_drop]
impl PinnedDrop for ValuesStream {
    fn drop(self: Pin<&mut Self>) {
        let this = self.project();
        if let Some(handle) = this.adv_handle.take() {
            drop(handle);
            log::trace!("Handle dropped for Advertisement");
        };
        if let Some(handle) = this.app_handle.take() {
            drop(handle);
            log::trace!("Handle dropped for Application");
        };
    }
}

impl futures::Stream for ValuesStream {
    type Item = Message;
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        self.project().values_rx.poll_next(cx)
    }
}

// pub fn create_runtime() -> Result<tokio::runtime::Runtime> {
//     tokio::runtime::Builder::new_current_thread().enable_all().build().map_err(|_| lipl_display_common::Error::Runtime)
// }

async fn first_adapter() -> Result<bluer::Adapter> {
    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;
    let adapter_name = adapter_names.first().ok_or(lipl_display_common::Error::NoBluetooth)?;
    let adapter: bluer::Adapter = session.adapter(adapter_name)?;
    adapter.set_powered(true).await?;
    Ok(adapter)
}

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

pub fn listen_background(cb: impl Fn(Message) -> Result<()> + Send + 'static) {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().map_err(lipl_display_common::Error::IO)?;

        runtime.block_on(async move {
            let mut s = listen_stream().await?.boxed();
            while let Some(message) = s.next().await {
                cb(message.clone())?;
                if message == Message::Command(Command::Exit) || message == Message::Command(Command::Poweroff) {
                    break;
                }
            }
            Ok::<(), Error>(())
        })
    });
}

pub async fn listen_stream() -> Result<impl Stream<Item=Message>> {
    let (values_tx, values_rx) = mpsc::channel::<Message>(100);

    let adapter = first_adapter().await?;
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
        ValuesStream {
            values_tx,
            values_rx,
            adv_handle: Some(adv_handle),
            app_handle: Some(app_handle),       
        }
    )
}
