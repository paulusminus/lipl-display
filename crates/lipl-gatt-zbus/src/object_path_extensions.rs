use std::convert::TryFrom;
use zbus::zvariant::OwnedObjectPath;

pub trait OwnedObjectPathExtensions {
    fn to_owned_object_path(&self) -> OwnedObjectPath;
}

impl<T: AsRef<str>> OwnedObjectPathExtensions for T {
    fn to_owned_object_path(&self) -> OwnedObjectPath {
        OwnedObjectPath::try_from(self.as_ref()).unwrap()
    }
}
