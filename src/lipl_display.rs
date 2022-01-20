use std::borrow::Cow;
use std::sync::mpsc::{Receiver};

use eframe::egui::{
    CtxRef,
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
pub const FONT_SMALL_FACTOR: f32 = 0.7;
pub const TEXT_DEFAULT: &str = "Even geduld a.u.b. ...";

pub struct LiplDisplay {
    pub text: Option<String>,
    pub status: Option<String>,
    pub config: LiplDisplayConfig,
    pub receiver: Receiver<Message>,
}

impl LiplDisplay {
    pub fn new(receiver: Receiver<Message>) -> Self {
        LiplDisplay {
            text: Some(TEXT_DEFAULT.to_owned()),
            status: None,
            receiver,
            config: Default::default(),
        }
    }
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
    pub fn configure_fonts(&self, ctx: &CtxRef) {
        let mut font_defs = FontDefinitions::default();
        font_defs.font_data.insert(
            FONT_NAME.to_owned(),
            FontData { font: Cow::Borrowed(FONT), index: 0 }
        );

        font_defs.family_and_size.insert(
            TextStyle::Body,
            (FontFamily::Proportional, self.config.font_size)
        );

        font_defs.family_and_size.insert(
            TextStyle::Small,
            (FontFamily::Proportional, self.config.font_size * FONT_SMALL_FACTOR)
        );

        font_defs.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(
            0,
            FONT_NAME.to_owned(),
        );

        ctx.set_fonts(font_defs);
    }

    pub fn configure_visuals(&self, ctx: &CtxRef) {
        ctx.set_visuals(crate::visuals::visuals(self.config.dark));
    }

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
        ui.add_space(self.config.font_size * FONT_SMALL_FACTOR);
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
        ui.add_space(self.config.font_size * FONT_SMALL_FACTOR);
    }
}
