use std::collections::HashMap;

use zbus::{ObjectPath, Result, Variant, proxy};

#[proxy(
    interface = "org.bluez.LEAdvertisingManager1",
    default_service = "org.bluez"
)]
pub trait LEAdvertisingManager1 {
    /// RegisterAdvertisement method
    fn register_advertisement(
        &self,
        advertisement: &ObjectPath<'_>,
        options: HashMap<&str, Variant<'_>>,
    ) -> Result<()>;

    /// UnregisterAdvertisement method
    fn unregister_advertisement(&self, service: &ObjectPath<'_>) -> Result<()>;

    /// ActiveInstances property
    #[zbus(property)]
    fn active_instances(&self) -> Result<u8>;

    /// SupportedIncludes property
    #[zbus(property)]
    fn supported_includes(&self) -> Result<Vec<String>>;

    /// SupportedInstances property
    #[zbus(property)]
    fn supported_instances(&self) -> Result<u8>;

    /// SupportedSecondaryChannels property
    #[zbus(property)]
    fn supported_secondary_channels(&self) -> Result<Vec<String>>;
}
