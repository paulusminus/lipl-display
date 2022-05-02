mod lipl_display;
mod visuals;

use eframe::{
    egui::{
        CentralPanel,
        Context,
        TopBottomPanel, TextStyle,
    },
    App,
    Frame,
    run_native,
    NativeOptions,
};
use lipl_display::{LiplDisplay};
use lipl_gatt_bluer::message::{Command, Message};

pub const TEXT_DEFAULT: &str = "Even geduld a.u.b. ...";
pub const FONT_SMALL_FACTOR: f32 = 0.7;

impl LiplDisplay {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<Message>();
        lipl_gatt_bluer::listen_background(move |message| tx.send(message).map_err(|_| lipl_gatt_bluer::Error::Callback));
    
        let config: crate::lipl_display::LiplDisplayConfig = Default::default();

        cc.egui_ctx.set_visuals(crate::visuals::visuals(config.dark));
        cc.egui_ctx.set_fonts(crate::lipl_display::configure_fonts());

        let mut style = (*cc.egui_ctx.style()).clone(); 
        style.text_styles.insert(TextStyle::Body, eframe::epaint::FontId { size: config.font_size, family: Default::default() });
        style.text_styles.insert(TextStyle::Small, eframe::epaint::FontId { size: config.font_size * FONT_SMALL_FACTOR, family: Default::default() });


        cc.egui_ctx.set_style(style);
        LiplDisplay {
            text: Some(TEXT_DEFAULT.to_owned()),
            status: None,
            receiver: rx,
            config,
        }
    }
}

impl App for LiplDisplay {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {

        ctx.request_repaint();

        if let Ok(value) = self.receiver.try_recv() {
            match value {
                Message::Part(text) => { self.text = Some(text); },
                Message::Status(text) => { self.status = Some(text); },
                Message::Command(command) => {
                    match command {
                        Command::Dark => { self.config.dark = true; ctx.set_visuals(crate::visuals::visuals(self.config.dark)); },
                        Command::Light => { self.config.dark = false; ctx.set_visuals(crate::visuals::visuals(self.config.dark)); },
                        Command::Increase => { self.config.font_size += 3.0; },
                        Command::Decrease => { if self.config.font_size > 5.0 { self.config.font_size -= 3.0; }; },
                        Command::Exit => { frame.quit(); },
                        Command::Poweroff => { 
                            frame.quit();
                        },
                    }
                }
            };
        }

        TopBottomPanel::bottom("Status").max_height(3. * (self.config.font_size * FONT_SMALL_FACTOR)).show(
            ctx,
            |ui | self.render_status(ui),
        );

        CentralPanel::default().show(
            ctx,
            |ui| self.render_text(ui),
        );
    }
}

fn fullscreen() -> NativeOptions {
    NativeOptions {
        maximized: true,
        decorated: false,
        ..Default::default()
    }
}

fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    run_native(
        "Lipl Display", 
        fullscreen(), 
        Box::new(|cc| Box::new(LiplDisplay::new(cc))),
    );
}
