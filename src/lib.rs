use bluer::{
    adv::{Advertisement, AdvertisementHandle},
    gatt::local::{
        Application,
        ApplicationHandle,
        Characteristic,
        Service,
    },
};
use std::{collections::BTreeMap, sync::Arc};
use futures::{channel::mpsc, Stream};
use tokio::{sync::Mutex};
use bluer::Uuid;
use tracing::{trace};
use pin_project_lite::pin_project;

mod constant;
pub mod message;
mod error;
mod characteristic;

pub use error::{Result, Error};

async fn first_adapter() -> Result<bluer::Adapter> {
    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;
    let adapter_name = adapter_names.first().ok_or_else(|| Error::NoBluetooth)?;
    let adapter: bluer::Adapter = session.adapter(adapter_name)?;
    adapter.set_powered(true).await?;
    Ok(adapter)
}

async fn advertise(adapter: &bluer::Adapter) -> Result<AdvertisementHandle> {
    let mut manufacturer_data = BTreeMap::new();
    manufacturer_data.insert(constant::MANUFACTURER_ID, vec![0x21, 0x22, 0x23, 0x24]);
    let le_advertisement = Advertisement {
        service_uuids: vec![constant::SERVICE_UUID.parse::<Uuid>().unwrap()].into_iter().collect(),
        manufacturer_data,
        discoverable: Some(true),
        local_name: Some(constant::LOCAL_NAME.to_owned()),
        ..Default::default()
    };
    let adv_handle = adapter.advertise(le_advertisement).await?;
    Ok(adv_handle)
}

pub fn create_channel() -> (mpsc::Sender<message::Message>, mpsc::Receiver<message::Message>) {
    mpsc::channel(10)
}

pub fn create_cancel() -> (mpsc::Sender<()>, mpsc::Receiver<()>) {
    mpsc::channel(1)
}

// pub async fn listen(mut cancel: mpsc::Receiver<()>, values_tx: mpsc::Sender<message::Message>) -> Result<()> {
//     let adapter = first_adapter().await?;
//     trace!("Bluetooth adapter {} found", adapter.name());
//     let adv_handle = advertise(&adapter).await?;
//     trace!("Advertising started");
        
//     let uuid: Uuid = constant::SERVICE_UUID.parse::<Uuid>().unwrap();
//     let primary: bool = true;
//     let characteristics: Vec<Characteristic> = 
//         constant::CHARACTERISTICS
//         .iter()
//         .map(|s| s.parse::<Uuid>().unwrap())
//         .map(|c| (c, Arc::new(Mutex::new(vec![]))))
//         .map(|v| characteristic::write_no_response_characteristic(v.0, v.1, values_tx.clone()))
//         .collect();
        
//     let app = Application {
//         services: vec![Service {
//             uuid,
//             primary,
//             characteristics,
//             ..Default::default()
//         }],
//         ..Default::default()
//     };
//     let app_handle: ApplicationHandle = adapter.serve_gatt_application(app).await?;
//     trace!("Gatt application started");
        
//     cancel.next().await;
//     trace!("Closing request received");
        
//     drop(app_handle);
//     drop(adv_handle);
//     time::sleep(Duration::from_millis(10)).await;
//     trace!("Finished");
//     Ok(())        
// }

pin_project! {
    struct ValuesStream {
        values_tx: futures::channel::mpsc::Sender<message::Message>,
        #[pin]
        values_rx: futures::channel::mpsc::Receiver<message::Message>,
        adv_handle: AdvertisementHandle,
        app_handle: ApplicationHandle,
    }    
}

// impl Drop for ValuesStream {
//     fn drop(&mut self) {
//         if let Some(app_handle) = &self.app_handle {
//             drop(app_handle);
//         }
//         if let Some(adv_handle) = &self.adv_handle {
//             drop(adv_handle);
//         }
//     }
// }

impl futures::Stream for ValuesStream {
    type Item = message::Message;
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        self.project().values_rx.poll_next(cx)
    }
}

pub async fn listen_stream() -> Result<impl Stream<Item=message::Message>> {
    let (values_tx, values_rx) = mpsc::channel::<message::Message>(100);

    let adapter = first_adapter().await?;
    trace!("Bluetooth adapter {} found", adapter.name());

    let adv_handle = advertise(&adapter).await?;
    trace!("Advertising started");        
    let uuid: Uuid = constant::SERVICE_UUID.parse::<Uuid>().unwrap();
    let primary: bool = true;
    let characteristics: Vec<Characteristic> = 
        constant::CHARACTERISTICS
        .iter()
        .map(|s| s.parse::<Uuid>().unwrap())
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
            adv_handle,
            app_handle,       
        }
    )
}
