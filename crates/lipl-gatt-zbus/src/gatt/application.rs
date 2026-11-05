use std::collections::HashMap;

use zbus::interface;
use zbus::{OwnedObjectPath, OwnedVariant};

#[derive(Clone, Debug)]
pub struct Application {
    pub objects: HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedVariant>>>,
}

#[interface(name = "org.freedesktop.DBus.ObjectManager")]
impl Application {
    // #[zbus(name = "GetManagedObjects")]
    fn get_managed_objects(
        &self,
    ) -> HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedVariant>>> {
        self.objects.clone()
    }
}
