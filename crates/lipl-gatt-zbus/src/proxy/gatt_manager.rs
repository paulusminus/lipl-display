use std::collections::HashMap;

use zbus::{ObjectPath, Result, Variant, proxy};

#[proxy(interface = "org.bluez.GattManager1", default_service = "org.bluez")]
pub trait GattManager1 {
    /// RegisterApplication method
    fn register_application(
        &self,
        application: &ObjectPath<'_>,
        options: HashMap<&str, Variant<'_>>,
    ) -> Result<()>;

    /// UnregisterApplication method
    fn unregister_application(&self, application: &ObjectPath<'_>) -> Result<()>;
}
