#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod font_size;
mod part;
mod status;
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
use part::Part;
use status::Status;
use std::time::Duration;
use theme::Theme;
use tokio::time::sleep;

mod constant;
// mod file_input;

async fn background_task() -> Result<(), std::io::Error> {
    #[cfg(feature = "fake")]
    let f = INPUT;
    #[cfg(not(feature = "fake"))]
    let f = json_lines::file_reader(PATH).await?;
    json_lines::lines(f)
        .try_for_each(move |message| {
            match message {
                Message::Part(p) => {
                    use_context::<Signal<Part>>().set(p.into());
                }
                Message::Status(s) => use_context::<Signal<Status>>().set(s.into()),
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
                        font_size.set((f + FONT_SIZE_INCREMENT).into());
                    }
                    Command::Decrease => {
                        let font_size = use_context::<Signal<FontSize>>().peek().value();
                        if font_size > MINIMUM_FONT_SIZE {
                            use_context::<Signal<FontSize>>()
                                .set((font_size - FONT_SIZE_INCREMENT).into());
                        }
                    }
                    Command::Wait => {
                        use_context::<Signal<Part>>().set("".to_owned().into());
                        use_context::<Signal<Status>>().set(WAIT_MESSAGE.to_owned().into());
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

#[component]
fn Root() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let font_size = use_context::<Signal<FontSize>>();
    let status = use_context::<Signal<Status>>();
    let part = use_context::<Signal<Part>>();

    use_future(background_task);

    rsx!(
        rect {
            width: "100%",
            height: "90%",
            background: theme().bg_color(),
            color: theme().fg_color(),
            font_size: font_size.to_string(),
            padding: "20",
            label {
                {part.to_string()}
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
                {status.to_string()}
            }
        }
    )
}

fn app() -> Element {
    use_platform().set_fullscreen_window(true);
    use_context_provider(|| Signal::new(Theme::dark));
    use_context_provider(|| Signal::new(|| FontSize::from(22)));
    use_context_provider(|| Signal::new(|| Status::from(WAIT_MESSAGE.to_owned())));
    use_context_provider(|| Signal::new(|| Part::from("".to_owned())));

    rsx!(Root {})
}

fn main() {
    launch_with_props(
        app,
        APPLICATION_TITLE,
        (APPLICATION_WIDTH, APPLICATION_HEIGHT),
    );
}
