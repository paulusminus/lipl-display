mod lipl_display;
mod visuals;

use eframe::{
    egui::{
        CentralPanel,
        CtxRef,
        TopBottomPanel,
    },
    epi::{
        App,
        Frame,
        Storage,
    },
    run_native,
    NativeOptions,
};
use lipl_display::{LiplDisplay};
use lipl_gatt_bluer::message::{Command, Message};

impl App for LiplDisplay {
    fn setup(&mut self, ctx: &CtxRef, _frame: &Frame, _storage: Option<&dyn Storage>) {
        self.configure_fonts(ctx);
        self.configure_visuals(ctx);
    }

    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {

        ctx.request_repaint();

        if let Ok(value) = self.receiver.try_recv() {
            match value {
                Message::Part(text) => { self.text = Some(text); },
                Message::Status(text) => { self.status = Some(text); },
                Message::Command(command) => {
                    match command {
                        Command::Dark => { self.config.dark = true; self.configure_visuals(ctx); },
                        Command::Light => { self.config.dark = false; self.configure_visuals(ctx); },
                        Command::Increase => { self.config.font_size += 3.0; self.configure_fonts(ctx); },
                        Command::Decrease => { if self.config.font_size > 5.0 { self.config.font_size -= 3.0; }; self.configure_fonts(ctx); },
                        Command::Exit => { frame.quit(); },
                        Command::Poweroff => { 
                            frame.quit();
                        },
                    }
                }
            };
        }

        TopBottomPanel::bottom("Status").max_height(3. * (self.config.font_size * lipl_display::FONT_SMALL_FACTOR)).show(
            ctx,
            |ui | self.render_status(ui),
        );

        CentralPanel::default().show(
            ctx,
            |ui| self.render_text(ui),
        );
    }

    fn name(&self) -> &str {
        "Lipl Display"
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

    let (tx, rx) = std::sync::mpsc::channel::<Message>();
    lipl_gatt_bluer::listen_background(move |message| tx.send(message).map_err(|_| lipl_gatt_bluer::Error::Callback));

    let app: LiplDisplay = LiplDisplay::new(rx);
    run_native(Box::new(app), fullscreen());
}
