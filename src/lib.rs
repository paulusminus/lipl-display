use bluer::{
    adv::{Advertisement, AdvertisementHandle},
    gatt::local::{
        Application,
        ApplicationHandle,
        Characteristic,
        Service,
    },
};
use std::{collections::BTreeMap, sync::Arc, time::Duration};
use futures::channel::{mpsc};
use futures::StreamExt;
use tokio::{sync::Mutex, time};
use bluer::Uuid;
use tracing::{trace};

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
        local_name: Some(std::env::var("HOSTNAME").map_err(|_| Error::Hostname)?),
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

pub async fn listen(mut cancel: mpsc::Receiver<()>, values_rx: mpsc::Sender<message::Message>) -> Result<()> {
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
        .map(|v| characteristic::write_no_response_characteristic(v.0, v.1, values_rx.clone()))
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
    let app_handle: ApplicationHandle = adapter.serve_gatt_application(app).await?;
    trace!("Gatt application started");
        
    cancel.next().await;
    trace!("Closing request received");
        
    drop(app_handle);
    drop(adv_handle);
    time::sleep(Duration::from_millis(10)).await;
    trace!("Finished");
    Ok(())        
}
