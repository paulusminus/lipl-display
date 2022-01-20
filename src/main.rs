use anyhow::{Result};
use gtk::prelude::*;
use gtk::glib::MainContext;
use lipl_gatt_bluer::{message::{Command, Message}};
use tracing::{trace};

mod css;
mod cursor;
mod window;

fn build_ui(application: &gtk::Application) -> Result<()> 
{
    css::load(css::Theme::Dark);
    let (values_tx, values_rx) = MainContext::channel::<Message>(gtk::glib::source::PRIORITY_DEFAULT);

    let mut app_window = window::AppWindow::new(application)?;
    let window_clone = app_window.clone();

    lipl_gatt_bluer::listen_background(move |message| {
        values_tx.send(message).map_err(|_| lipl_gatt_bluer::Error::Callback)
    });

    values_rx.attach(None, move |value| {
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

        gtk::glib::Continue(true)
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
