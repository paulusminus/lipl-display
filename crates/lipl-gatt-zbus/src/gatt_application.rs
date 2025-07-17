use crate::gatt::{Characteristic, Request, Service};
use derive_builder::Builder;
use futures::channel::mpsc::Sender;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub(crate) struct GattApplication {
    pub local_name: String,
    pub app_object_path: String,
    pub services: Vec<Service>,
    pub characteristics: Vec<Characteristic>,
}

#[derive(Builder, Clone, Debug, Default)]
pub struct GattServiceConfig {
    #[builder(default = "true")]
    pub primary: bool,
    pub uuid: Uuid,
    pub characteristics: Vec<GattCharacteristicConfig>,
}

#[derive(Builder, Clone, Debug, Default)]
pub struct GattCharacteristicConfig {
    pub uuid: Uuid,
    #[builder(default = "false")]
    pub read: bool,
    #[builder(default = "true")]
    pub write: bool,
}

#[derive(Builder, Clone, Debug, Default)]
pub struct GattApplicationConfig {
    #[builder(default = "\"lipl\".to_string()")]
    pub local_name: String,
    #[builder(default = "\"/org/bluez/app\".to_string()")]
    pub app_object_path: String,
    pub services: Vec<GattServiceConfig>,
}

impl From<(GattApplicationConfig, Sender<Request>)> for GattApplication {
    fn from(config: (GattApplicationConfig, Sender<Request>)) -> Self {
        let mut services = vec![];
        let mut characteristics = vec![];

        for (service_index, service_config) in config.0.services.iter().enumerate() {
            let service_object_path =
                format!("{}/service{}", config.0.app_object_path, service_index + 1);

            let service_characteristics = service_config
                .characteristics
                .iter()
                .enumerate()
                .map(|gatt_char_config| {
                    Characteristic::from((
                        gatt_char_config.0,
                        gatt_char_config.1,
                        service_object_path.clone(),
                        config.1.clone(),
                        service_config.uuid,
                    ))
                })
                .collect::<Vec<_>>();

            let service = Service {
                object_path: service_object_path,
                primary: service_config.primary,
                uuid: service_config.uuid,
                characteristic_paths: service_characteristics
                    .iter()
                    .map(|p| p.object_path.clone())
                    .collect(),
            };
            services.push(service);
            characteristics.extend(service_characteristics);
        }

        Self {
            local_name: config.0.local_name,
            app_object_path: config.0.app_object_path,
            services,
            characteristics,
        }
    }
}
