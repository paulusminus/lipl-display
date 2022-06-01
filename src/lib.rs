use bluer::{
    adv::{Advertisement, AdvertisementHandle},
    gatt::local::{
        Application,
        ApplicationHandle,
        Characteristic,
        Service,
    },
};
use message::{Command, Message};
use std::{collections::BTreeMap, sync::Arc};
use futures::{channel::mpsc, Stream, StreamExt};
use tokio::{sync::Mutex};
use bluer::Uuid;
use log::{trace};
use pin_project::{pin_project, pinned_drop};
use std::pin::Pin;

mod constant;
pub mod message;
mod error;
mod characteristic;

pub use error::{Result, Error};

#[pin_project(PinnedDrop)]
struct ValuesStream {
    values_tx: futures::channel::mpsc::Sender<message::Message>,
    #[pin]
    values_rx: futures::channel::mpsc::Receiver<message::Message>,
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
    type Item = message::Message;
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        self.project().values_rx.poll_next(cx)
    }
}

pub fn create_runtime() -> Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_current_thread().enable_all().build().map_err(|_| Error::Runtime)
}

async fn first_adapter() -> Result<bluer::Adapter> {
    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;
    let adapter_name = adapter_names.first().ok_or(Error::NoBluetooth)?;
    let adapter: bluer::Adapter = session.adapter(adapter_name)?;
    adapter.set_powered(true).await?;
    Ok(adapter)
}

async fn advertise(adapter: &bluer::Adapter) -> Result<AdvertisementHandle> {
    let mut manufacturer_data = BTreeMap::new();
    manufacturer_data.insert(constant::MANUFACTURER_ID, vec![0x21, 0x22, 0x23, 0x24]);
    let le_advertisement = Advertisement {
        service_uuids: vec![constant::SERVICE_UUID].into_iter().collect(),
        manufacturer_data,
        discoverable: Some(true),
        local_name: Some(constant::LOCAL_NAME.to_owned()),
        ..Default::default()
    };
    adapter.advertise(le_advertisement).await.map_err(Error::from)
}

pub fn listen_background(cb: impl Fn(Message) -> Result<()> + Send + 'static) {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;

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

pub async fn listen_stream() -> Result<impl Stream<Item=message::Message>> {
    let (values_tx, values_rx) = mpsc::channel::<Message>(100);

    let adapter = first_adapter().await?;
    trace!("Bluetooth adapter {} found", adapter.name());

    let adv_handle = advertise(&adapter).await?;
    trace!("Advertising started");        
    let uuid: Uuid = constant::SERVICE_UUID;
    let primary: bool = true;
    let characteristics: Vec<Characteristic> = 
        constant::CHARACTERISTICS
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
