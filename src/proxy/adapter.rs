use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.bluez.Adapter1",
    default_service = "org.bluez"
)]
pub trait Adapter1 {
    /// GetDiscoveryFilters method
    fn get_discovery_filters(&self) -> zbus::Result<Vec<String>>;

    /// RemoveDevice method
    fn remove_device(&self, device: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// SetDiscoveryFilter method
    fn set_discovery_filter(
        &self,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// StartDiscovery method
    fn start_discovery(&self) -> zbus::Result<()>;

    /// StopDiscovery method
    fn stop_discovery(&self) -> zbus::Result<()>;

    /// Address property
    #[dbus_proxy(property)]
    fn address(&self) -> zbus::Result<String>;

    /// AddressType property
    #[dbus_proxy(property)]
    fn address_type(&self) -> zbus::Result<String>;

    /// Alias property
    #[dbus_proxy(property)]
    fn alias(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_alias(&self, value: &str) -> zbus::Result<()>;

    /// Class property
    #[dbus_proxy(property)]
    fn class(&self) -> zbus::Result<u32>;

    /// Discoverable property
    #[dbus_proxy(property)]
    fn discoverable(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_discoverable(&self, value: bool) -> zbus::Result<()>;

    /// DiscoverableTimeout property
    #[dbus_proxy(property)]
    fn discoverable_timeout(&self) -> zbus::Result<u32>;
    #[dbus_proxy(property)]
    fn set_discoverable_timeout(&self, value: u32) -> zbus::Result<()>;

    /// Discovering property
    #[dbus_proxy(property)]
    fn discovering(&self) -> zbus::Result<bool>;

    /// ExperimentalFeatures property
    #[dbus_proxy(property)]
    fn experimental_features(&self) -> zbus::Result<Vec<String>>;

    /// Modalias property
    #[dbus_proxy(property)]
    fn modalias(&self) -> zbus::Result<String>;

    /// Name property
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::Result<String>;

    /// Pairable property
    #[dbus_proxy(property)]
    fn pairable(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_pairable(&self, value: bool) -> zbus::Result<()>;

    /// PairableTimeout property
    #[dbus_proxy(property)]
    fn pairable_timeout(&self) -> zbus::Result<u32>;
    #[dbus_proxy(property)]
    fn set_pairable_timeout(&self, value: u32) -> zbus::Result<()>;

    /// Powered property
    #[dbus_proxy(property)]
    fn powered(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_powered(&self, value: bool) -> zbus::Result<()>;

    /// Roles property
    #[dbus_proxy(property)]
    fn roles(&self) -> zbus::Result<Vec<String>>;

    /// UUIDs property
    #[dbus_proxy(property)]
    fn uuids(&self) -> zbus::Result<Vec<String>>;
}
