use std::{sync::mpsc::{Receiver}};

use eframe::egui::{
    FontDefinitions,
    FontFamily,
    Label,
    Layout,
    TextStyle,
    Direction,
    FontData, RichText,
};

use lipl_gatt_bluer::message::Message;

pub const FONT: &[u8] = include_bytes!("Roboto-Regular.ttf");
pub const FONT_NAME: &str = "Roboto";
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


pub fn configure_fonts() -> FontDefinitions {
    let mut font_defs = FontDefinitions::default();
    font_defs.font_data.insert(
        FONT_NAME.to_owned(),
        FontData::from_static(FONT),
    );

    font_defs.families.get_mut(&FontFamily::Proportional).unwrap().insert(
        0,
        FONT_NAME.to_owned(),
    );

    font_defs.families.get_mut(&FontFamily::Proportional).unwrap().insert(
        0,
        FONT_NAME.to_owned(),
    );

    font_defs

}


impl LiplDisplay {

    // pub fn configure_visuals(&self, ctx: &Context) {
    //     ctx.set_visuals(crate::visuals::visuals(self.config.dark));
    // }

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
        ui.add_space(self.config.font_size * crate::FONT_SMALL_FACTOR);
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
        ui.add_space(self.config.font_size * crate::FONT_SMALL_FACTOR);
    }
}
