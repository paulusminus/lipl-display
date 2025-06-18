#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod theme;

#[cfg(feature = "fake")]
use constant::INPUT;
#[cfg(not(feature = "fake"))]
use constant::PATH;
use constant::{
    APPLICATION_HEIGHT, APPLICATION_TITLE, APPLICATION_WIDTH, FONT_SIZE_INCREMENT,
    MINIMUM_FONT_SIZE, WAIT_MESSAGE,
};
use freya::launch::launch_with_props;
use freya::prelude::*;
use futures_util::{FutureExt, TryStreamExt};
use lipl_display_common::{Command, Message};
use std::time::Duration;
use theme::Theme;
use tokio::time::sleep;

mod constant;
// mod file_input;

async fn background_task(
    mut part: Signal<String>,
    mut status: Signal<String>,
    mut theme: Signal<Theme>,
    mut font_size: Signal<usize>,
) -> Result<(), std::io::Error> {
    #[cfg(feature = "fake")]
    let f = INPUT;
    #[cfg(not(feature = "fake"))]
    let f = json_lines::file_reader(PATH).await?;
    json_lines::lines(f)
        .try_for_each(move |message| {
            match message {
                Message::Part(p) => {
                    part.set(p);
                }
                Message::Status(s) => status.set(s),
                Message::Command(c) => match c {
                    Command::Dark => {
                        theme.set(Theme::dark());
                    }
                    Command::Light => {
                        theme.set(Theme::light());
                    }
                    Command::Increase => {
                        font_size.set(font_size() + FONT_SIZE_INCREMENT);
                    }
                    Command::Decrease => {
                        if font_size() > MINIMUM_FONT_SIZE {
                            font_size.set(font_size() - FONT_SIZE_INCREMENT);
                        }
                    }
                    Command::Wait => {
                        part.set("".to_owned());
                        status.set(WAIT_MESSAGE.to_owned());
                    }
                    Command::Exit => {
                        use_platform().exit();
                    }
                    _ => {}
                },
            }
            sleep(Duration::from_secs(1)).map(|_| Ok(()))
        })
        .await
}

fn app() -> Element {
    use_platform().set_fullscreen_window(true);

    let part = use_signal(|| "".to_string());
    let status = use_signal(|| WAIT_MESSAGE.to_string());
    let theme = use_signal(|| Theme::light());
    let font_size = use_signal(|| 22usize);

    use_future(move || background_task(part, status, theme, font_size));
    rsx!(
        rect {
            width: "100%",
            height: "90%",
            background: theme().bg_color(),
            color: theme().fg_color(),
            font_size: font_size,
            padding: "20",
            label {
                {part}
            }
        }
        rect {
            width: "100%",
            height: "10%",
            background: theme().bg_color(),
            color: theme().fg_color(),
            padding: "20",
            font_size: font_size,
            label {
                {status}
            }
        }
    )
}

fn main() {
    launch_with_props(
        app,
        APPLICATION_TITLE,
        (APPLICATION_WIDTH, APPLICATION_HEIGHT),
    );
}
