use eframe::egui::{FontDefinitions, FontData, FontFamily};

pub const FONT: &[u8] = include_bytes!("Roboto-Regular.ttf");
pub const FONT_NAME: &str = "Roboto";

pub fn fonts() -> FontDefinitions {
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
