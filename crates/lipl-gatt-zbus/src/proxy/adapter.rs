use std::collections::HashMap;

use zbus::{ObjectPath, Result, Variant, proxy};

#[proxy(interface = "org.bluez.Adapter1", default_service = "org.bluez")]
pub trait Adapter1 {
    /// GetDiscoveryFilters method
    fn get_discovery_filters(&self) -> Result<Vec<String>>;

    /// RemoveDevice method
    fn remove_device(&self, device: &ObjectPath<'_>) -> Result<()>;

    /// SetDiscoveryFilter method
    fn set_discovery_filter(&self, properties: HashMap<&str, Variant<'_>>) -> Result<()>;

    /// StartDiscovery method
    fn start_discovery(&self) -> Result<()>;

    /// StopDiscovery method
    fn stop_discovery(&self) -> Result<()>;

    /// Address property
    #[zbus(property)]
    fn address(&self) -> Result<String>;

    /// AddressType property
    #[zbus(property)]
    fn address_type(&self) -> Result<String>;

    /// Alias property
    #[zbus(property)]
    fn alias(&self) -> Result<String>;

    #[zbus(property)]
    fn set_alias(&self, value: &str) -> Result<()>;

    /// Class property
    #[zbus(property)]
    fn class(&self) -> Result<u32>;

    /// Discoverable property
    #[zbus(property)]
    fn discoverable(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_discoverable(&self, value: bool) -> Result<()>;

    /// DiscoverableTimeout property
    #[zbus(property)]
    fn discoverable_timeout(&self) -> Result<u32>;

    #[zbus(property)]
    fn set_discoverable_timeout(&self, value: u32) -> Result<()>;

    /// Discovering property
    #[zbus(property)]
    fn discovering(&self) -> Result<bool>;

    /// ExperimentalFeatures property
    #[zbus(property)]
    fn experimental_features(&self) -> Result<Vec<String>>;

    /// Modalias property
    #[zbus(property)]
    fn modalias(&self) -> Result<String>;

    /// Name property
    #[zbus(property)]
    fn name(&self) -> Result<String>;

    /// Pairable property
    #[zbus(property)]
    fn pairable(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_pairable(&self, value: bool) -> Result<()>;

    /// PairableTimeout property
    #[zbus(property)]
    fn pairable_timeout(&self) -> Result<u32>;

    #[zbus(property)]
    fn set_pairable_timeout(&self, value: u32) -> Result<()>;

    /// Powered property
    #[zbus(property)]
    fn powered(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_powered(&self, value: bool) -> Result<()>;

    /// Roles property
    #[zbus(property)]
    fn roles(&self) -> Result<Vec<String>>;

    /// UUIDs property
    #[zbus(property)]
    fn uuids(&self) -> Result<Vec<String>>;
}
