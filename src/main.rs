use anyhow::{Result};
use futures::{StreamExt};
use gtk::gio::prelude::*;
use gtk::glib::MainContext;
use lipl_gatt_bluer::{listen, message::{Command, Message}};
use gtk::prelude::WidgetExt;
use tracing::{trace};

mod css;
mod cursor;
mod window;

fn build_ui(application: &gtk::Application) -> Result<()> 
{
    css::load(css::Theme::Dark);
    let (cancel_tx, cancel_rx) = lipl_gatt_bluer::create_cancel();
    let cancel_tx_clone = std::sync::Mutex::new(Some(cancel_tx));
    let (values_tx, mut values_rx) = lipl_gatt_bluer::create_channel();
    let _background = std::thread::spawn(move || {
        trace!("Background task started");
        if let Ok(runtime) = tokio::runtime::Builder::new_current_thread().enable_all().build() {
            let _result = runtime.block_on(listen(cancel_rx, values_tx));
        };

    });

    let mut app_window = window::AppWindow::new(application)?;
    let window_clone = app_window.clone();

    app_window.window.connect_delete_event(move |_,_| {
        if let Some(mut tx) = cancel_tx_clone.lock().unwrap().take() {
            let _result = tx.try_send(());
        }
        gtk::Inhibit(false)
    });

    MainContext::default().spawn_local(async move {
        while let Some(value) = values_rx.next().await {
            match value {
                Message::Part(s) => {
                    app_window.set_text(&s);
                    trace!("Text updated");
                },
                Message::Status(s) => {
                    app_window.set_status(&s);
                    trace!("Status updated");
                },
                Message::Command(command) => {
                    match command {
                        Command::Increase => {
                            app_window.increase_font_size();
                            trace!("Increase font size");
                        },
                        Command::Decrease => {
                            app_window.decrease_font_size();
                            trace!("Decrease font size");
                        },
                        Command::Light => {
                            css::load(css::Theme::Light);
                            trace!("Light theme");
                        },
                        Command::Dark => {
                            css::load(css::Theme::Dark);
                            trace!("Dark theme");
                        },
                        Command::Exit => {
                            window_clone.close();
                            trace!("Exit");
                        },
                        Command::Poweroff => {
                            window_clone.close();
                            trace!("Poweroff");
                        }
                    }
                }
            }
        }
    });

    Ok(())
}

fn main() -> anyhow::Result<()> {

    let subscriber = tracing_subscriber::FmtSubscriber::builder().with_max_level(tracing::Level::TRACE).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to initialize logging");

    let application: gtk::Application = 
        gtk::builders::ApplicationBuilder::new()
        .application_id("nl.paulmin.lipl.display")
        .flags(Default::default())
        .build();


    application.connect_activate(move |app| {
        if let Err(err) = build_ui(app) {
            eprintln!("{}", err);
        }
    });

    application.run();
    Ok(())
}
