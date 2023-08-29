mod adapter;
#[allow(non_snake_case)]
mod device;
mod le_advertising_manager;
mod gatt_manager;


pub(crate) use adapter::Adapter1Proxy;
pub(crate) use device::Device1Proxy;
pub(crate) use gatt_manager::GattManager1Proxy;
pub(crate) use le_advertising_manager::LEAdvertisingManager1Proxy;