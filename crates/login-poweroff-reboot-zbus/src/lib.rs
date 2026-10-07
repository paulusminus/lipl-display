#![doc = include_str!("../README.md")]

use manager::ManagerProxy;
use std::time::{SystemTime, UNIX_EPOCH};
use strum::Display;

mod manager;

#[derive(Display)]
#[strum(serialize_all = "lowercase")]
pub enum Shutdown {
    Poweroff,
    Reboot,
}

struct ZbusConnection<'a> {
    _connection: zbus::Connection,
    proxy: ManagerProxy<'a>,
}

impl<'a> ZbusConnection<'a> {
    async fn new() -> zbus::Result<Self> {
        let _connection = zbus::Connection::system().await?;
        let proxy = manager::ManagerProxy::builder(&_connection).build().await?;
        Ok(Self { _connection, proxy })
    }
    fn manager(&self) -> &ManagerProxy<'a> {
        &self.proxy
    }
}

/// Converts a delay from now in milliseconds to milliseconds since the Unix epoch.
fn time(delay_millis: u64) -> zbus::Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|now| now.as_millis() as u64 + delay_millis)
        .map_err(|_| zbus::Error::Unsupported)
}

/// Reboots the machine.
pub async fn schedule_shutdown(typ: Shutdown, delay_millis: u64) -> zbus::Result<()> {
    ZbusConnection::new()
        .await?
        .manager()
        .schedule_shutdown(&typ.to_string(), time(delay_millis)?)
        .await
}
