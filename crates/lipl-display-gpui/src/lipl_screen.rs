use async_channel::Receiver;
use gpui::{AppContext, AsyncApp, Entity, Hsla, WeakEntity};
use lipl_display_common::{Command, Message};
use std::cmp::max;
use ui::Pixels;

use crate::constant::{DARK, DEFAULT_STATUS, INITIAL_FONT_SIZE, MIN_FONT_SIZE};

fn update(lipl_screen_weak: WeakEntity<LiplScreen>, cx: &mut gpui::App) {
    if let Some(lipl_screen) = lipl_screen_weak.upgrade().as_ref() {
        cx.update_entity(lipl_screen, |lipl_screen, _| {})
        // ...
    }
}

pub fn init(cx: &mut gpui::App, receiver: Receiver<Message>) -> Entity<LiplScreen> {
    let lipl_screen = cx.new(|_| LiplScreen::new(DARK, INITIAL_FONT_SIZE));
    let _ = cx.observe(&lipl_screen, |_, cx| {
        cx.refresh_windows();
    });
    let lipl_screen_weak = lipl_screen.downgrade();
    cx.spawn(async move |cx: &mut AsyncApp| {
        while let Ok(message) = receiver.recv().await {
            match message {
                Message::Part(part) => {
                    // Process the message
                    lipl_screen_weak.upgrade().as_ref().is_some_and(|display| {
                        cx.update_entity(display, |display, _| {
                            display.set_text(&part);
                        })
                        .is_ok()
                    });
                    if let Some(display) = lipl_screen_weak.upgrade().as_ref() {
                        cx.update_entity(display, |display, _| {
                            display.set_text(&part);
                        });
                    }
                }
                Message::Status(status) => {
                    // Process the message
                    cx.update_entity(&display, |display, _| {
                        display.set_status(&status);
                    })
                    .unwrap();
                }
                Message::Command(command) => {
                    match command {
                        Command::Dark => {
                            // Process the message
                            cx.update_entity(&display, |display, _| {
                                display.set_dark(true);
                            })
                            .unwrap();
                        }
                        Command::Light => {
                            // Process the message
                            cx.update_entity(&display, |display, _| {
                                display.set_dark(false);
                            })
                            .unwrap();
                        }
                        Command::Exit => {}
                        Command::Poweroff => {
                            // Process the message
                        }
                        Command::Increase => {
                            cx.update_entity(&display, |display, _| {
                                display.increase_font_size();
                            })
                            .unwrap();
                        }
                        Command::Decrease => {
                            cx.update_entity(&display, |display, _| {
                                display.decrease_font_size();
                            })
                            .unwrap();
                        }
                        Command::Wait => {
                            cx.update_entity(&display, |display, _| {
                                display.set_text("");
                                display.set_status(DEFAULT_STATUS)
                            })
                            .unwrap();
                        }
                    }
                }
            }
            cx.refresh().unwrap();
        }
    })
    .detach();
    lipl_screen
}

pub struct LiplScreen(lipl_display_common::LiplScreen);

impl LiplScreen {
    pub fn new(dark: bool, initial_fontsize: f32) -> Self {
        Self(lipl_display_common::LiplScreen::new(dark, initial_fontsize))
    }
    pub fn background_color(&self) -> Hsla {
        if self.0.dark {
            Hsla::black()
        } else {
            Hsla::white()
        }
    }

    pub fn foreground_color(&self) -> Hsla {
        if self.0.dark {
            Hsla::white()
        } else {
            Hsla::black()
        }
    }
    pub fn text(&self) -> String {
        self.0.text.clone()
    }
    pub fn status(&self) -> String {
        self.0.status.clone()
    }
    pub fn set_text(&mut self, text: &str) {
        self.0.text = text.into();
    }
    pub fn set_status(&mut self, status: &str) {
        self.0.status = status.into();
    }
    pub fn set_dark(&mut self, dark: bool) {
        self.0.dark = dark;
    }
    pub fn font_size(&self) -> Pixels {
        (self.0.font_size as usize).into()
    }

    pub fn font_size_status(&self) -> Pixels {
        (max(self.0.font_size as usize - 2, MIN_FONT_SIZE)).into()
    }
    pub fn increase_font_size(&mut self) {
        self.0.font_size += 1.0;
    }
    pub fn decrease_font_size(&mut self) {
        if self.0.font_size as usize > MIN_FONT_SIZE {
            self.0.font_size -= 1.0;
        }
    }
}
