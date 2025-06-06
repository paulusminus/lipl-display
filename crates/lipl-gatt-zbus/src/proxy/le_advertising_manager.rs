use zbus::proxy;

#[proxy(
    interface = "org.bluez.LEAdvertisingManager1",
    default_service = "org.bluez"
)]
pub trait LEAdvertisingManager1 {
    /// RegisterAdvertisement method
    fn register_advertisement(
        &self,
        advertisement: &zbus::zvariant::ObjectPath<'_>,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// UnregisterAdvertisement method
    fn unregister_advertisement(
        &self,
        service: &zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// ActiveInstances property
    #[zbus(property)]
    fn active_instances(&self) -> zbus::Result<u8>;

    /// SupportedIncludes property
    #[zbus(property)]
    fn supported_includes(&self) -> zbus::Result<Vec<String>>;

    /// SupportedInstances property
    #[zbus(property)]
    fn supported_instances(&self) -> zbus::Result<u8>;

    /// SupportedSecondaryChannels property
    #[zbus(property)]
    fn supported_secondary_channels(&self) -> zbus::Result<Vec<String>>;
}
