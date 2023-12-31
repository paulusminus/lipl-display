use std::sync::mpsc::Receiver;

use eframe::egui::{Direction, Label, Layout, RichText, TextStyle};

use lipl_display_common::Message;

pub const FONT_SIZE: f32 = 40.;

pub struct LiplDisplay {
    pub text: Option<String>,
    pub status: Option<String>,
    pub config: LiplDisplayConfig,
    pub receiver: Receiver<Message>,
}

pub struct LiplDisplayConfig {
    pub font_size: f32,
    pub dark: bool,
}

impl Default for LiplDisplayConfig {
    fn default() -> Self {
        LiplDisplayConfig {
            font_size: FONT_SIZE,
            dark: true,
        }
    }
}

impl LiplDisplay {
    pub fn render_text(&self, ui: &mut eframe::egui::Ui) {
        ui.with_layout(
            Layout::centered_and_justified(Direction::LeftToRight),
            |ui| {
                if let Some(text) = &self.text {
                    let label = Label::new(RichText::new(text).text_style(TextStyle::Body));
                    // let label = Label::new(text).text_style(TextStyle::Body);
                    ui.add(label);
                }
            },
        );
    }

    pub fn render_status(&self, ui: &mut eframe::egui::Ui) {
        ui.add_space(self.config.font_size * crate::style::FONT_SMALL_FACTOR);
        ui.with_layout(
            Layout::centered_and_justified(Direction::LeftToRight),
            |ui| {
                if let Some(text) = &self.status {
                    let label = Label::new(RichText::new(text).text_style(TextStyle::Small));
                    // let label = Label::new(text).text_style(TextStyle::Small);
                    ui.add(label);
                }
            },
        );
        ui.add_space(self.config.font_size * crate::style::FONT_SMALL_FACTOR);
    }
}
