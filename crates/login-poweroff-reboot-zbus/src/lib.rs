#![doc = include_str!("../README.md")]

use zbus::Connection;

mod manager;

/// Reboots the machine.
pub async fn reboot() -> zbus::Result<()> {
    let connection = Connection::session().await?;
    let manager = manager::ManagerProxy::builder(&connection).build().await?;
    manager.reboot(false).await
}

/// Powers off the machine.
pub async fn poweroff() -> zbus::Result<()> {
    let connection = Connection::session().await?;
    let manager = manager::ManagerProxy::builder(&connection).build().await?;
    manager.power_off(false).await
}
