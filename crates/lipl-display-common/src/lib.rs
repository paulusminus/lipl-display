use std::str::FromStr;
use std::convert::TryFrom;
use uuid::{uuid, Uuid};

mod error;

/// Error type
pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub trait HandleMessage {
    fn handle_message(&self, message: Message) -> Self;
}

/// Uuid identifying the display service on the gatt peripheral
pub const SERVICE_UUID: Uuid = uuid!("27a70fc8-dc38-40c7-80bc-359462e4b808");
/// Local name used in advertising
pub const LOCAL_NAME: &str = "lipl";
/// Manufacturer id used in advertising
pub const MANUFACTURER_ID: u16 = 0xffff;

/// Uuid identifying the text characteristic on the gatt peripheral
pub const CHARACTERISTIC_TEXT_UUID: Uuid = uuid!("04973569-c039-4ce9-ad96-861589a74f9e");
/// Uuid identifying the status characteristic on the gatt peripheral
pub const CHARACTERISTIC_STATUS_UUID: Uuid = uuid!("61a8cb7f-d4c1-49b7-a3cf-f2c69dbb7aeb");
/// Uuid identifying the command characteristic on the gatt peripheral
pub const CHARACTERISTIC_COMMAND_UUID: Uuid = uuid!("da35e0b2-7864-49e5-aa47-8050d1cc1484");


pub trait Listen {
    fn listen_background(&mut self, cb: impl Fn(Message) + Send + 'static);
    fn stop(&mut self);
}

/// Received value on the display service as change for the screen
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    Part(String),
    Status(String),
    Command(Command),
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Message::Part(text) => format!("Text: {}", text),
                Message::Status(status) => format!("Status: {}", status ),
                Message::Command(command) => format!("Command: {}", command),
            }
        )
    }
}

/// Received value from command characteristic
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    Poweroff,
    Exit,
    Increase,
    Decrease,
    Light,
    Dark,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Command::Dark => "d",
                Command::Light => "l",
                Command::Increase => "+",
                Command::Decrease => "-",
                Command::Exit => "e",
                Command::Poweroff => "o", 
            }
        )
    }
}

impl FromStr for Command {
    type Err = error::Error; 
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {

        if s.is_empty() {
            return Err(
                error::Error::GattCharaceristicValueParsing("Empty".into())
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
            error::Error::GattCharaceristicValueParsing(
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
            return s.parse::<Command>().map(Message::Command);
        }

        Err(Error::GattCharaceristicValueParsing(s))
    }
}

#[derive(Clone)]
pub struct Part {
    pub text: String,
    pub status: String,
    pub dark: bool,
    pub font_size: f32,
}

impl Part {
    pub fn new(dark: bool, initial_text: String, initial_font_size: f32) -> Self {
        Self {
            text: initial_text,
            status: "".to_owned(),
            dark,
            font_size: initial_font_size,
        }
    }
}

impl HandleMessage for Part {
    fn handle_message(&self, message: Message) -> Self {
        match message {
            Message::Command(command) => {
                match command {
                    Command::Dark => Self {
                        dark: true,
                        ..self.clone()
                    },
                    Command::Light => Self {
                        dark: false,
                        ..self.clone()
                    },
                    Command::Decrease => Self {
                        font_size: (self.font_size - 1.0).max(2.0),
                        ..self.clone()
                    },
                    Command::Increase => Self {
                        font_size: self.font_size + 1.0,
                        ..self.clone()
                    },
                    _ => self.clone(),
                }
            },
            Message::Part(part) => Self {
                text: part,
                ..self.clone()
            },
            Message::Status(status) => Self {
                status,
                ..self.clone()
            },
        }
    }
}