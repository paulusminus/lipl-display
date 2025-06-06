use crate::object_path_extensions::OwnedObjectPathExtensions;
use uuid::Uuid;
use zbus::{interface, zvariant::OwnedObjectPath};

#[derive(Clone, Debug)]
pub struct Service {
    pub object_path: String,
    pub primary: bool,
    pub uuid: Uuid,
    pub characteristic_paths: Vec<String>,
}

#[interface(name = "org.bluez.GattService1")]
impl Service {
    #[zbus(property)]
    fn primary(&self) -> bool {
        self.primary
    }

    #[zbus(property, name = "UUID")]
    fn uuid(&self) -> String {
        self.uuid.to_string().to_uppercase()
    }

    #[zbus(property)]
    fn characteristics(&self) -> Vec<OwnedObjectPath> {
        self.characteristic_paths
            .iter()
            .map(|s| s.to_owned_object_path())
            .collect()
    }
}
