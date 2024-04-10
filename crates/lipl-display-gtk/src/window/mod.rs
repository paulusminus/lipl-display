use anyhow::{anyhow, Result};
use gtk4::prelude::*;

pub const TEXT_ID: &str = "text";
pub const PAGNOS_ID: &str = "pagenos";
pub const WINDOW_ID: &str = "window";
pub const TEXT_INIT: &str = "Even geduld a.u.b. ...";
pub const WINDOW_UI: &str = include_str!("window.ui");

#[derive(Clone)]
pub struct Data {
    pub text: String,
    pub status: String,
    pub font_size: u16,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            text: TEXT_INIT.to_owned(),
            status: "".to_owned(),
            font_size: 40,
        }
    }
}

#[derive(Clone)]
pub struct AppWindow {
    pub window: gtk4::ApplicationWindow,
    pub text: gtk4::Label,
    pub status: gtk4::Label,
    data: Data,
}

impl AppWindow {
    pub fn new(application: &gtk4::Application) -> Result<Self> {
        let builder = gtk4::Builder::from_string(WINDOW_UI);
        // let mut cancel_clone = cancel.clone();

        let window: gtk4::ApplicationWindow = builder
            .object(WINDOW_ID)
            .ok_or_else(|| anyhow!("Missing Window control"))?;
        let text: gtk4::Label = builder
            .object(TEXT_ID)
            .ok_or_else(|| anyhow!("Missing text control"))?;
        let status: gtk4::Label = builder
            .object(PAGNOS_ID)
            .ok_or_else(|| anyhow!("Missing pagnos control"))?;

        window.set_application(Some(application));
        window.fullscreen();
        window.show();

        crate::cursor::hide(&window);

        let app_window = AppWindow {
            window,
            text,
            status,
            data: Default::default(),
        };

        Ok(app_window)
    }

    pub fn set_text(&mut self, text: &str) {
        self.data.text = text.to_owned();
        self.update_text_label();
    }

    pub fn set_status(&mut self, text: &str) {
        self.data.status = text.to_owned();
        self.update_status_label();
    }

    pub fn increase_font_size(&mut self) {
        self.data.font_size += 1;
        self.refresh();
    }

    pub fn decrease_font_size(&mut self) {
        self.data.font_size -= 1;
        self.refresh();
    }

    fn refresh(&self) {
        self.update_status_label();
        self.update_text_label();
    }

    fn update_text_label(&self) {
        self.text.set_markup(&format!(
            "<span font=\"{}\">{}</span>",
            self.data.font_size, self.data.text
        ));
    }

    fn update_status_label(&self) {
        self.status.set_markup(&format!(
            "<span font=\"{}\">{}</span>",
            self.data.font_size / 2,
            self.data.status
        ));
    }

    pub fn close(&self) {
        self.window.close();
    }
}
