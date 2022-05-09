use std::str::FromStr;
// use serde::{Serialize};

#[derive(Clone, Debug, PartialEq)]
// #[serde(rename_all = "lowercase")]
pub enum Message {
    Part(String),
    Status(String),
    Command(Command),
}

#[derive(Clone, Debug, PartialEq)]
// #[serde(rename_all = "lowercase")]
pub enum Command {
    Poweroff,
    Exit,
    Increase,
    Decrease,
    Light,
    Dark,
}

impl FromStr for Command {
    type Err = (); 
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.is_empty() {
            return Err(());
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

        Err(())
    }
}
