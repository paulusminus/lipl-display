use crate::LiplDisplay;
use lipl_display_common::{Command, Message};
use slint::{invoke_from_event_loop, quit_event_loop, Weak};
use tracing::error;

pub(crate) fn create_handle_message(ui_handle: Weak<LiplDisplay>) -> impl Fn(Message) {
    move |message| match message {
        Message::Part(part) => {
            let handle_copy = ui_handle.clone();
            if let Err(error) =
                invoke_from_event_loop(move || handle_copy.unwrap().set_part(part.into()))
            {
                error!("Error handling received part {}", error);
            };
        }
        Message::Status(status) => {
            let handle_copy = ui_handle.clone();
            if let Err(error) =
                invoke_from_event_loop(move || handle_copy.unwrap().set_status(status.into()))
            {
                error!("Error handling received status {}", error);
            };
        }
        Message::Command(command) => match command {
            Command::Dark => {
                let handle_copy = ui_handle.clone();
                if let Err(error) =
                    invoke_from_event_loop(move || handle_copy.unwrap().set_dark(true))
                {
                    error!("Error handling set theme dark {}", error);
                };
            }
            Command::Light => {
                let handle_copy = ui_handle.clone();
                if let Err(error) =
                    invoke_from_event_loop(move || handle_copy.unwrap().set_dark(false))
                {
                    error!("Error handling set theme light {}", error);
                };
            }
            Command::Increase => {
                let handle_copy = ui_handle.clone();
                if let Err(error) = invoke_from_event_loop(move || {
                    let ui = handle_copy.unwrap();
                    let length = ui.get_fontsize();
                    ui.set_fontsize(length + 2);
                }) {
                    error!("Failed to handle increase command {}", error);
                }
            }
            Command::Decrease => {
                let handle_copy = ui_handle.clone();
                if let Err(error) = invoke_from_event_loop(move || {
                    let ui = handle_copy.unwrap();
                    let length = ui.get_fontsize();
                    if length > 4 {
                        ui.set_fontsize(length - 2)
                    };
                }) {
                    error!("Failed to handle decrease command {error}");
                }
            }
            Command::Exit => {
                if let Err(error) = quit_event_loop() {
                    error!("Failed to handle exit command {error}");
                }
            }
            Command::Poweroff => {
                if let Err(error) = login_poweroff_reboot::poweroff(5000) {
                    error!("Failed to send poweroff to systemd-logind: {error}");
                }

                if let Err(error) = quit_event_loop() {
                    error!("Failed to handle exit command {error}");
                }
            }
            Command::Wait => {
                let handle_copy = ui_handle.clone();
                if let Err(error) = invoke_from_event_loop(move || {
                    let screen = handle_copy.unwrap();
                    screen.set_status(lipl_display_common::WAIT_MESSAGE.into());
                    screen.set_part("".into());
                }) {
                    error!("Error handling received status {}", error);
                };
            }
        },
    }
}
