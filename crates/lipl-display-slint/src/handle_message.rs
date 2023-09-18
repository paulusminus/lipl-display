use lipl_display_common::{Message, Command};
use log::error;
use crate::LiplDisplay;
use slint::{Weak, ComponentHandle};

pub(crate) fn create_handle_message(ui_handle: Weak<LiplDisplay>) -> impl Fn(Message) {
    move |message| {
        match message {
            Message::Part(part) => {
                let handle_copy = ui_handle.clone();
                if let Err(error) = slint::invoke_from_event_loop(move || handle_copy.unwrap().set_part(part.into())) {
                    error!("Error handling received part {}", error);
                };
            },
            Message::Status(status) => {
                let handle_copy = ui_handle.clone();
                if let Err(error) = slint::invoke_from_event_loop(move || handle_copy.unwrap().set_status(status.into())) {
                    error!("Error handling received status {}", error);
                };
            },
            Message::Command(command) => {
                match command {
                    Command::Dark => {
                        let handle_copy = ui_handle.clone();
                        if let Err(error) = slint::invoke_from_event_loop(move || handle_copy.unwrap().set_dark(true)) {
                            error!("Error handling set theme dark {}", error);
                        };
                    },
                    Command::Light => {
                        let handle_copy = ui_handle.clone();
                        if let Err(error) = slint::invoke_from_event_loop(move || handle_copy.unwrap().set_dark(false)) {
                            error!("Error handling set theme light {}", error);
                        };
                    },
                    Command::Increase => {
                        let handle_copy = ui_handle.clone();
                        if let Err(error) = slint::invoke_from_event_loop(move || {
                            let ui = handle_copy.unwrap();
                            let length = ui.get_fontsize();
                            ui.set_fontsize(length + 2);
                        }) {
                            error!("Failed to handle increase command {}", error);
                        }
                    },
                    Command::Decrease => {
                        let handle_copy = ui_handle.clone();
                        if let Err(error) = slint::invoke_from_event_loop(move || {
                            let ui = handle_copy.unwrap();
                            let length = ui.get_fontsize();
                            if length > 4 { ui.set_fontsize(length - 2) };
                        }) {
                            error!("Failed to handle decrease command {}", error);
                        }
                    },
                    Command::Exit => {
                        let handle_copy = ui_handle.clone();
                        if let Err(error) = slint::invoke_from_event_loop(move || 
                            handle_copy.unwrap().window().dispatch_event(slint::platform::WindowEvent::CloseRequested)
                        ) {
                            error!("Failed to handle exit command {}", error);
                        }
                    },
                    Command::Poweroff => {
                        let handle_copy = ui_handle.clone();
                        if let Err(error) = slint::invoke_from_event_loop(move || 
                            handle_copy.unwrap().window().dispatch_event(slint::platform::WindowEvent::CloseRequested)
                        ) {
                            error!("Failed to handle poweroff command {}", error);
                        }
                    }
                }
            }
        }
    }
}
