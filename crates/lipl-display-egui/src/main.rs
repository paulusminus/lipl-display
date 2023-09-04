mod fonts;
mod lipl_display;
mod style;
mod visuals;

use std::sync::mpsc::{Sender, Receiver};

use eframe::{
    egui::{
        CentralPanel,
        Context,
        TopBottomPanel,
    },
    App,
    Frame,
    run_native,
    NativeOptions,
};
use lipl_display::LiplDisplay;
use lipl_display_common::{BackgroundThread, Command, Message};
use lipl_gatt_bluer::ListenBluer;

const TEXT_DEFAULT: &str = "Even geduld a.u.b. ...";

fn create_callback(tx: Sender<Message>) -> impl Fn(Message) {
    move |message| {
        if let Err(error) = tx.send(message) {
            log::error!("Error sending message: {}", error);
        }
    }
}

impl LiplDisplay {
    fn new(cc: &eframe::CreationContext<'_>, rx: Receiver<Message>) -> Self {
        // gatt.listen_background(
        //     move |message| 
        //         if let Err(error) = tx.send(message) {
        //             log::error!("Error sending message: {}", error);
        //         }
        // );

        cc.egui_ctx.set_fonts(fonts::fonts());
    
        let config: lipl_display::LiplDisplayConfig = Default::default();

        visuals::set_dark_mode(&cc.egui_ctx, config.dark);
        style::set_font_size(&cc.egui_ctx, config.font_size);

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
                        Command::Dark => { self.config.dark = true; visuals::set_dark_mode(ctx, self.config.dark); },
                        Command::Light => { self.config.dark = false; visuals::set_dark_mode(ctx, self.config.dark); },
                        Command::Increase => { self.config.font_size += 3.0; style::set_font_size(ctx, self.config.font_size) },
                        Command::Decrease => { if self.config.font_size > 5.0 { self.config.font_size -= 3.0; style::set_font_size(ctx, self.config.font_size) }; },
                        Command::Exit => { frame.close(); },
                        Command::Poweroff => { 
                            frame.close();
                        },
                    }
                }
            };
        }

        TopBottomPanel::bottom("Status").max_height(3. * (self.config.font_size * style::FONT_SMALL_FACTOR)).show(
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

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();

    let (tx, rx) = std::sync::mpsc::channel::<Message>();
    let mut gatt = ListenBluer::new(create_callback(tx));

    run_native(
        "Lipl Display", 
        fullscreen(), 
        Box::new(|cc| Box::new(LiplDisplay::new(cc, rx))),
    )
    .map_err(|_| anyhow::anyhow!("Error running egui"))?;
    gatt.stop();
    Ok(())
}
