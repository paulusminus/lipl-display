use lipl_display_common::{Message, Command};
use crate::LiplDisplay;
use slint::{Weak, ComponentHandle};

pub(crate) fn create_handle_message(ui_handle: Weak<LiplDisplay>) -> impl Fn(Message) {
    move |message| {
        match message {
            Message::Part(part) => {
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_part(part.into());
                }
            },
            Message::Status(status) => {
                if let Some(ui) = ui_handle.upgrade() {
                    ui.set_status(status.into());
                }
            },
            Message::Command(command) => {
                match command {
                    Command::Dark => {
                        if let Some(ui) = ui_handle.upgrade() {
                            ui.set_dark(true);
                        }
                    },
                    Command::Light => {
                        if let Some(ui) = ui_handle.upgrade() {
                            ui.set_dark(false);
                        }
                    },
                    Command::Increase => {
                        if let Some(ui) = ui_handle.upgrade() {
                            let length = ui.get_whatever();
                            ui.set_whatever(length + 2);
                        }
                    },
                    Command::Decrease => {
                        if let Some(ui) = ui_handle.upgrade() {
                            let length = ui.get_whatever();
                            if length > 4 { ui.set_whatever(length - 2); }
                        }
                    },
                    Command::Exit => {
                        if let Some(ui) = ui_handle.upgrade() {
                            ui.window().dispatch_event(slint::platform::WindowEvent::CloseRequested);
                        }
                    },
                    Command::Poweroff => {
                        if let Some(ui) = ui_handle.upgrade() {
                            ui.window().dispatch_event(slint::platform::WindowEvent::CloseRequested);
                        }
                    }
                }
            }
        }
    }
}
