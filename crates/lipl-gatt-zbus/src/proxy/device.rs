use std::collections::HashMap;

use zbus::{OwnedObjectPath, OwnedVariant, Result, proxy};

#[proxy(interface = "org.bluez.Device1", default_service = "org.bluez")]
pub trait Device1 {
    /// CancelPairing method
    fn cancel_pairing(&self) -> Result<()>;

    /// Connect method
    fn connect(&self) -> Result<()>;

    /// ConnectProfile method
    fn connect_profile(&self, UUID: &str) -> Result<()>;

    /// Disconnect method
    fn disconnect(&self) -> Result<()>;

    /// DisconnectProfile method
    fn disconnect_profile(&self, UUID: &str) -> Result<()>;

    /// Pair method
    fn pair(&self) -> Result<()>;

    /// Adapter property
    #[zbus(property)]
    fn adapter(&self) -> Result<OwnedObjectPath>;

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

    /// Appearance property
    #[zbus(property)]
    fn appearance(&self) -> Result<u16>;

    /// Blocked property
    #[zbus(property)]
    fn blocked(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_blocked(&self, value: bool) -> Result<()>;

    /// Class property
    #[zbus(property)]
    fn class(&self) -> Result<u32>;

    /// Connected property
    #[zbus(property)]
    fn connected(&self) -> Result<bool>;

    /// Icon property
    #[zbus(property)]
    fn icon(&self) -> Result<String>;

    /// LegacyPairing property
    #[zbus(property)]
    fn legacy_pairing(&self) -> Result<bool>;

    /// ManufacturerData property
    #[zbus(property)]
    fn manufacturer_data(&self) -> Result<HashMap<u16, OwnedVariant>>;

    /// Modalias property
    #[zbus(property)]
    fn modalias(&self) -> Result<String>;

    /// Name property
    #[zbus(property)]
    fn name(&self) -> Result<String>;

    /// Paired property
    #[zbus(property)]
    fn paired(&self) -> Result<bool>;

    /// RSSI property
    #[zbus(property)]
    fn rssi(&self) -> Result<i16>;

    /// ServiceData property
    #[zbus(property)]
    fn service_data(&self) -> Result<HashMap<String, OwnedVariant>>;

    /// ServicesResolved property
    #[zbus(property)]
    fn services_resolved(&self) -> Result<bool>;

    /// Trusted property
    #[zbus(property)]
    fn trusted(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_trusted(&self, value: bool) -> Result<()>;

    /// TxPower property
    #[zbus(property)]
    fn tx_power(&self) -> Result<i16>;

    /// UUIDs property
    #[zbus(property)]
    fn uuids(&self) -> Result<Vec<String>>;

    /// WakeAllowed property
    #[zbus(property)]
    fn wake_allowed(&self) -> Result<bool>;

    #[zbus(property)]
    fn set_wake_allowed(&self, value: bool) -> Result<()>;
}
