use uuid::Uuid;

use crate::gatt::{Service, Characteristic};

#[derive(Clone, Debug)]
pub(crate) struct GattApplication {
    pub local_name: String,
    pub app_object_path: String,
    pub services: Vec<Service>,
    pub characteristics: Vec<Characteristic>,
}


pub struct GattCharacteristicConfig {
    pub uuid: Uuid,
}

pub struct GattServiceConfig {
    pub primary: bool,
    pub uuid: Uuid,
    pub characteristics: Vec<GattCharacteristicConfig>,
}

pub struct GattApplicationConfig {
    pub local_name: String,
    pub app_object_path: String,
    pub services: Vec<GattServiceConfig>,
}

impl From<GattApplicationConfig> for GattApplication {
    fn from(config: GattApplicationConfig) -> Self {
        let mut services = vec![];
        let mut characteristics = vec![];

        let mut service_index = 0;

        for service_config in config.services {
            service_index += 1;
            let service_object_path = format!("{}/service{}", config.app_object_path, service_index);

            let service_characteristics = 
                service_config
                .characteristics
                .iter()
                .enumerate()
                .map(
                    |config| 
                        Characteristic::new_write_only(
                            format!("{}/char{}", service_object_path, config.0 + 1),
                            config.1.uuid,
                            service_object_path.clone(),
                        )
                )
                .collect::<Vec<_>>();

            let service = Service {
                object_path: service_object_path,
                primary: service_config.primary,
                uuid: service_config.uuid,
                characteristic_paths: service_characteristics.iter().map(|p| p.object_path.clone()).collect(),
            };
            services.push(service);
            characteristics.extend(service_characteristics);
        }

        Self { 
            local_name: config.local_name,
            app_object_path: config.app_object_path,
            services, 
            characteristics,
        }
    }   
}