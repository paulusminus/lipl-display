use std::collections::HashMap;

use zbus::{
    interface,
    zvariant::{OwnedObjectPath, OwnedValue},
};

#[derive(Clone, Debug)]
pub struct Application {
    pub objects: HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>>,
}

#[interface(name = "org.freedesktop.DBus.ObjectManager")]
impl Application {
    #[zbus(name = "GetManagedObjects")]
    fn get_managed_objects(
        &self,
    ) -> HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>> {
        self.objects.clone()
    }
}
