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
use futures::{StreamExt};
use lipl_display::{LiplDisplay};
use lipl_gatt_bluer::message::{Command, Message};
use tracing::{trace, error};

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
                            match self.close.try_send(()) {
                                Ok(_) => {
                                    frame.quit();
                                },
                                Err(e) => { error!("Error poweroff: {}", e); }
                            };
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
    let subscriber = tracing_subscriber::FmtSubscriber::builder().with_max_level(tracing::Level::TRACE).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to initialize logging");

    let (tx, rx) = std::sync::mpsc::channel::<Message>();
    let (cancel_tx, cancel_rx) = lipl_gatt_bluer::create_cancel();
    std::thread::spawn(move || {
        tracing::trace!("Background thread started");

        let (values_tx, mut values_rx) = lipl_gatt_bluer::create_channel();

        let runtime = 
            tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap();
        

        runtime.block_on(async move {
            let task1 = async move {
                trace!("Receiving values from Gatt Application");
                while let Some(value) = values_rx.next().await {
                    tx.send(value)?;
                }
                Ok::<(), Box<dyn std::error::Error>>(())
            };
    
            let task2 = async move {
                trace!("Start listening on gatt peripheral");
                lipl_gatt_bluer::listen(cancel_rx, values_tx).await?;
                Ok::<(), Box<dyn std::error::Error>>(())
            };
    
            match tokio::try_join!(task1, task2) {
                Ok(_) => {},
                Err(e) => { error!("Error: {}", e); }
            };
        });

    });

    let app: LiplDisplay = LiplDisplay::new(rx, cancel_tx);

    run_native(Box::new(app), fullscreen());
}
