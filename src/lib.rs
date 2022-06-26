use std::str::FromStr;
use std::convert::TryFrom;
use uuid::{uuid, Uuid};

pub mod error;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub const SERVICE_UUID: Uuid = uuid!("27a70fc8-dc38-40c7-80bc-359462e4b808");
pub const LOCAL_NAME: &str = "lipl";
pub const MANUFACTURER_ID: u16 = 0xffff;

pub const CHARACTERISTIC_TEXT_UUID: Uuid = uuid!("04973569-c039-4ce9-ad96-861589a74f9e");
pub const CHARACTERISTIC_STATUS_UUID: Uuid = uuid!("61a8cb7f-d4c1-49b7-a3cf-f2c69dbb7aeb");
pub const CHARACTERISTIC_COMMAND_UUID: Uuid = uuid!("da35e0b2-7864-49e5-aa47-8050d1cc1484");

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    Part(String),
    Status(String),
    Command(Command),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Poweroff,
    Exit,
    Increase,
    Decrease,
    Light,
    Dark,
}

impl FromStr for Command {
    type Err = error::Error; 
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {

        if s.is_empty() {
            return Err(
                error::Error::GattCharaceristicValueParsingFailed("Empty".into())
            );
        }

        if s == "+" {
            return Ok(Command::Increase);
        }

        if s == "-" {
            return Ok(Command::Decrease);
        }

        if s == "o" {
            return Ok(Command::Poweroff);
        }

        if s == "l" {
            return Ok(Command::Light);
        }

        if s == "d" {
            return Ok(Command::Dark);
        }

        if s == "e" {
            return Ok(Command::Exit);
        }

        Err(
            error::Error::GattCharaceristicValueParsingFailed(
                format!("Unknown command {s} received")
            )
        )
    }
}


impl TryFrom<(&str, Uuid)> for Message {
    type Error = Error;
    fn try_from(received: (&str, Uuid)) -> Result<Self> {
        let uuid = received.1;
        let s = received.0.to_owned();

        if uuid == CHARACTERISTIC_TEXT_UUID {
            return Ok(Message::Part(s));
        }

        if uuid == CHARACTERISTIC_STATUS_UUID {
            return Ok(Message::Status(s));
        }

        if uuid == CHARACTERISTIC_COMMAND_UUID {
            let command = received.0.parse::<Command>()?;
            return Ok(Message::Command(command));
        }

        Err(Error::GattCharaceristicValueParsingFailed(s))
    }
}