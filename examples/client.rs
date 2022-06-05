use std::collections::HashMap;

use zbus_bluez::{PeripheralConnection, Value};
use lipl_display_common::{SERVICE_UUID};

#[tokio::main(flavor = "current_thread")]
async fn main() -> zbus::Result<()> {
    println!("Hallo Paul");

    env_logger::init();

    let bluez = PeripheralConnection::new().await?;

    let adapter = bluez.adapter();

    // let services = Value::from(vec![SERVICE_UUID.to_string()]);
    let mut search: HashMap<&str, Value> = HashMap::new();
    search.insert("UUIDs", Value::from(vec![SERVICE_UUID.to_string()]));
    search.insert("Transport", Value::from("le".to_string()));
    adapter.set_discovery_filter(search).await?;
    adapter.start_discovery().await?;

    log::info!("Press ctrl-c to exit");
    adapter.stop_discovery().await?;
    tokio::signal::ctrl_c().await?;
    Ok(())
}
