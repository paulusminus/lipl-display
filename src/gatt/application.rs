use std::{collections::HashMap};

use zbus::{
    dbus_interface,
    zvariant::{
        OwnedObjectPath,
        OwnedValue
    },
};

#[derive(Clone, Debug)]
pub struct Application {
    pub objects: HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>>,
}

#[dbus_interface(name = "org.freedesktop.DBus.ObjectManager")]
impl Application {
    #[dbus_interface(name = "GetManagedObjects")]
    fn get_managed_objects(&self) -> HashMap<OwnedObjectPath, HashMap<String, HashMap<String, OwnedValue>>> {
        self.objects.clone()
    }
}
