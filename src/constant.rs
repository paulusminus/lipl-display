pub const SERVICE_UUID: &str = "27a70fc8-dc38-40c7-80bc-359462e4b808";
pub const LOCAL_NAME: &str = "lipl";
pub const MANUFACTURER_ID: u16 = 0xf00d;

pub const CHARACTERISTIC_TEXT_UUID: &str = "04973569-c039-4ce9-ad96-861589a74f9e";
pub const CHARACTERISTIC_STATUS_UUID: &str = "61a8cb7f-d4c1-49b7-a3cf-f2c69dbb7aeb";
pub const CHARACTERISTIC_COMMAND_UUID: &str = "da35e0b2-7864-49e5-aa47-8050d1cc1484";

pub const CHARACTERISTICS: &[&str] = &[CHARACTERISTIC_TEXT_UUID, CHARACTERISTIC_STATUS_UUID, CHARACTERISTIC_COMMAND_UUID];

pub const ERROR_NO_ADAPTER: &str = "No bluetooth adapter found";
