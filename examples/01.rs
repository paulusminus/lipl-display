use futures::TryFutureExt;
use zbus::zvariant::OwnedObjectPath;
use zbus_bluez::{Bluez, gatt_capable};

fn object_path_to_string(object_path: OwnedObjectPath) -> String {
    object_path.as_str().into()
}

fn print_adapter(adapter: String) {
    println!("Adapter: {adapter}");
}

fn print_adapters(adapters: impl Iterator<Item = OwnedObjectPath>) {
    adapters.map(object_path_to_string).for_each(print_adapter);
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> zbus::Result<()> {
    Bluez::new()
    .and_then(|bluez| async move { bluez.list_adapters(gatt_capable).await })
    .map_ok(print_adapters)
    .await   
}
