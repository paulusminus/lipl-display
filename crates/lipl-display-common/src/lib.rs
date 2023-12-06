#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use std::convert::TryFrom;
use std::str::FromStr;
use uuid::{uuid, Uuid};

mod error;

/// Error type
pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub trait HandleMessage {
    fn handle_message(&mut self, message: Message);
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

pub const WAIT_MESSAGE: &str = "Even geduld a.u.b. ...";

pub const MESSAGES: &[(&str, Command); 7] = &[
    ("d", Command::Dark),
    ("l", Command::Light),
    ("+", Command::Increase),
    ("-", Command::Decrease),
    ("?", Command::Wait),
    ("e", Command::Exit),
    ("o", Command::Poweroff),
];

pub trait BackgroundThread {
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
                Message::Status(status) => format!("Status: {}", status),
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
    Wait,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            MESSAGES.iter().find(|s| &s.1 == self).map(|s| s.0).unwrap()
        )
    }
}

impl FromStr for Command {
    type Err = error::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        MESSAGES
            .iter()
            .find(|t| t.0 == s)
            .map(|t| t.1.clone())
            .ok_or(error::Error::GattCharaceristicValueParsing(
                "Invalid command".to_owned(),
            ))
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

/// Model holding all that is needed to draw a screen
///
#[derive(Clone)]
pub struct LiplScreen {
    pub text: String,
    pub status: String,
    pub dark: bool,
    pub font_size: f32,
}

impl LiplScreen {
    /// LiplScreen constructor
    ///
    /// # Example
    ///
    /// ```
    /// use lipl_display_common::LiplScreen;
    /// let screen = LiplScreen::new(true, "Just a moment ...", 30.0);
    /// # assert!(screen.dark);
    /// # assert_eq!(screen.font_size, 30.0);
    /// ```
    pub fn new(dark: bool, initial_text: &str, initial_font_size: f32) -> Self {
        Self {
            text: initial_text.to_owned(),
            status: "".to_owned(),
            dark,
            font_size: initial_font_size,
        }
    }
}

impl HandleMessage for LiplScreen {
    //! Create a new screen with an update applied
    //!
    //! # Example
    //!
    //! ```
    //! use lipl_display_common::{Command, LiplScreen, HandleMessage, Message};
    //! let mut screen = LiplScreen::new(true, "", 40.0);
    //! assert!(screen.dark);
    //! screen.handle_message(Message::Command(Command::Light));
    //! assert!(!screen.dark);
    //! ```
    //!
    fn handle_message(&mut self, message: Message) {
        match message {
            Message::Command(command) => match command {
                Command::Dark => {
                    self.dark = true;
                }
                Command::Light => {
                    self.dark = false;
                }
                Command::Decrease => {
                    self.font_size = (self.font_size - 1.0).max(2.0);
                }
                Command::Increase => {
                    self.font_size = (self.font_size + 1.0).min(100.0);
                }
                Command::Wait => {
                    self.text = String::new();
                    self.status = WAIT_MESSAGE.to_owned();
                }
                Command::Exit => {}
                Command::Poweroff => {}
            },
            Message::Part(part) => {
                self.text = part;
            }
            Message::Status(status) => {
                self.status = status;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Command, MESSAGES};

    #[test]
    fn parse() {
        for message in MESSAGES {
            assert_eq!(message.0.parse::<Command>().unwrap(), message.1);
        }

        for message in MESSAGES {
            assert_eq!(message.1.to_string(), message.0.to_string());
        }
    }
}
