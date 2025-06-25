#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod font_size;
mod theme;

#[cfg(feature = "fake")]
use constant::INPUT;
#[cfg(not(feature = "fake"))]
use constant::PATH;
use constant::{
    APPLICATION_HEIGHT, APPLICATION_TITLE, APPLICATION_WIDTH, FONT_SIZE_INCREMENT,
    MINIMUM_FONT_SIZE, WAIT_MESSAGE,
};
use font_size::FontSize;
use freya::launch::launch_with_props;
use freya::prelude::*;
use futures_util::{FutureExt, TryStreamExt};
use lipl_display_common::{Command, Message};
use std::{ops::Deref, time::Duration};
use theme::Theme;
use tokio::time::sleep;

mod constant;
// mod file_input;

async fn background_task(
    mut part: Signal<String>,
    mut status: Signal<String>,
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
                        use_context::<Signal<Theme>>().set(Theme::dark());
                    }
                    Command::Light => {
                        use_context::<Signal<Theme>>().set(Theme::light());
                    }
                    Command::Increase => {
                        let mut font_size = use_context::<Signal<FontSize>>();
                        let f = font_size.peek().value();
                        font_size.set(*font_size.peek().deref() + FONT_SIZE_INCREMENT);
                    }
                    Command::Decrease => {
                        let font_size = { use_context::<Signal<FontSize>>().peek() };
                        if font_size.value() > MINIMUM_FONT_SIZE {
                            use_context::<Signal<FontSize>>()
                                .set((font_size.value() - FONT_SIZE_INCREMENT).into());
                        }
                    }
                    Command::Wait => {
                        part.set("".to_owned());
                        status.set(WAIT_MESSAGE.to_owned());
                    }
                    Command::Exit => {
                        use_platform().exit();
                    }
                    Command::Poweroff => {
                        use_platform().exit();
                    }
                },
            }
            sleep(Duration::from_secs(1)).map(|_| Ok(()))
        })
        .await
}

fn show_part() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let font_size = use_context::<Signal<FontSize>>();
    rsx!(
        rect {
            width: "100%",
            height: "90%",
            background: theme().bg_color(),
            color: theme().fg_color(),
            font_size: font_size.to_string(),
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
            font_size: font_size.to_string(),
            label {
                {status}
            }
        }
    )
}

fn app() -> Element {
    use_platform().set_fullscreen_window(true);
    use_context_provider(|| Signal::new(|| Theme::dark()));
    use_context_provider(|| Signal::new(|| FontSize::from(22)));
    let part = use_signal(|| "".to_string());
    let status = use_signal(|| WAIT_MESSAGE.to_string());
    let theme = use_signal(|| Theme::light());
    let font_size = use_signal(|| 22usize);

    use_future(move || background_task(part, status, font_size));
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
