use anyhow::{Result};
use gtk::prelude::*;
use gtk::glib::MainContext;
use lipl_gatt_bluer::{Command, Message};
use log::{trace};

mod css;
mod cursor;
mod window;

static GLIB_LOGGER: gtk::glib::GlibLogger = gtk::glib::GlibLogger::new(
    gtk::glib::GlibLoggerFormat::Plain,
    gtk::glib::GlibLoggerDomain::CrateTarget,
);

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

fn main() -> Result<()> {
    log::set_logger(&GLIB_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

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
