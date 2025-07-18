use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use async_channel::{Sender, bounded};
use glib::ExitCode;
use gtk4::{
    glib::clone,
    prelude::{ApplicationExt, ApplicationExtManual},
};
use lipl_display_common::{BackgroundThread, Command, Message};
use lipl_gatt_bluer::ListenBluer;
use log::{error, trace};

mod css;
mod cursor;
mod window;

static GLIB_LOGGER: gtk4::glib::GlibLogger = gtk4::glib::GlibLogger::new(
    gtk4::glib::GlibLoggerFormat::Plain,
    gtk4::glib::GlibLoggerDomain::CrateTarget,
);

fn create_callback(tx: Sender<Message>) -> impl Fn(Message) {
    move |message| {
        if let Err(error) = tx.send_blocking(message) {
            error!("Error sending message: {error}");
        }
    }
}

fn build_ui(application: &gtk4::Application) -> Result<()> {
    let (values_tx, values_rx) = bounded(1);
    let gatt = Rc::new(RefCell::new(ListenBluer::new(create_callback(values_tx))));

    css::load(css::Theme::Dark);

    let mut app_window = window::AppWindow::new(application)?;
    let window_clone = app_window.clone();

    application.connect_shutdown(clone!(
        #[strong]
        gatt,
        move |_| {
            gatt.borrow_mut().stop();
        }
    ));

    glib::spawn_future_local(async move {
        while let Ok(value) = values_rx.recv().await {
            match value {
                Message::Part(s) => {
                    app_window.set_text(&s);
                    trace!("Text updated");
                }
                Message::Status(s) => {
                    app_window.set_status(&s);
                    trace!("Status updated");
                }
                Message::Command(command) => match command {
                    Command::Increase => {
                        app_window.increase_font_size();
                        trace!("Increase font size");
                    }
                    Command::Decrease => {
                        app_window.decrease_font_size();
                        trace!("Decrease font size");
                    }
                    Command::Light => {
                        css::load(css::Theme::Light);
                        trace!("Light theme");
                    }
                    Command::Dark => {
                        css::load(css::Theme::Dark);
                        trace!("Dark theme");
                    }
                    Command::Exit => {
                        window_clone.close();
                        trace!("Exit");
                        break;
                    }
                    Command::Poweroff => {
                        window_clone.close();
                        trace!("Poweroff");
                        break;
                    }
                    Command::Wait => {
                        app_window.set_status(lipl_display_common::WAIT_MESSAGE);
                        app_window.set_text("");
                        trace!("Status Wait");
                    }
                },
            }
        }
    });

    Ok(())
}

fn main() -> Result<ExitCode> {
    log::set_logger(&GLIB_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    let application: gtk4::Application = gtk4::Application::builder()
        .application_id("nl.paulmin.lipl.display")
        .flags(Default::default())
        .build();

    application.connect_activate(move |app| {
        if let Err(err) = build_ui(app) {
            eprintln!("{err}");
        }
    });

    Ok(application.run())
}
