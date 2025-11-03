use futures_util::TryStreamExt;
use lipl_display_common::{Command, HandleMessage, LiplScreen, Message};
use masonry::widgets::GridParams;
use std::str;
use std::time::Duration;
use winit::error::EventLoopError;
use xilem::core::{MessageProxy, fork};
use xilem::style::{Background, Style};
use xilem::view::{
    Axis, CrossAxisAlignment, GridExt, MainAxisAlignment, flex, grid, label, sized_box, task,
};
use xilem::{Color, EventLoop, WidgetView, WindowOptions, Xilem, tokio};

const APP_TITLE: &str = "Lipl Display";
const WAIT_MESSAGE: &str = "Even geduld a.u.b. ...";
const ROBOTO_FONT: &[u8] = include_bytes!("/usr/share/fonts/google-roboto/Roboto-Regular.ttf");
const DEFAULT_DARK: bool = true;
const DEFAULT_FONT_SIZE: f32 = 22.0;

trait LiplScreenExt {
    fn bg_color(&self) -> Color;
    fn fg_color(&self) -> Color;
}

impl LiplScreenExt for LiplScreen {
    fn bg_color(&self) -> Color {
        if self.dark {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }

    fn fg_color(&self) -> Color {
        if self.dark {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }
}

fn display(screen: &mut LiplScreen) -> impl WidgetView<LiplScreen> + use<> {
    sized_box(grid(
        (
            flex(
                Axis::Horizontal,
                label(screen.text.clone())
                    .text_size(screen.font_size)
                    .font("Roboto")
                    .color(screen.fg_color()),
            )
            .direction(Axis::Horizontal)
            .main_axis_alignment(MainAxisAlignment::Center)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .grid_item(GridParams::new(0, 0, 1, 11)),
            flex(
                Axis::Horizontal,
                label(screen.status.as_str())
                    .text_size(screen.font_size)
                    .color(screen.fg_color()),
            )
            .direction(Axis::Horizontal)
            .main_axis_alignment(MainAxisAlignment::Center)
            .grid_pos(0, 11),
        ),
        1,
        12,
    ))
    .background(Background::Color(screen.bg_color()))
}

async fn background_task(proxy: MessageProxy<Message>) {
    let result = async {
        tracing::info!("Starting task");
        let r = json_lines::file_reader("/home/paul/Code/dart/lipl_display/lipl-gatt-input.txt")
            .await?;
        tracing::info!("input from file ok");
        let mut s = json_lines::lines::<Message>(r);
        while let Some(message) = s.try_next().await? {
            tracing::info!("Received message: {}", message);
            proxy.message(message)?;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        Ok::<(), Box<dyn std::error::Error>>(())
    }
    .await;
    match result {
        Ok(()) => {
            tracing::info!("Task completed successfully");
        }
        Err(error) => {
            tracing::error!("Task failed: {}", error);
        }
    }
}

fn on_message_received(screen: &mut LiplScreen, message: Message) {
    let handled = {
        if let Message::Command(command) = &message {
            if *command == Command::Wait {
                screen.text = String::default();
                screen.status = WAIT_MESSAGE.into();
                true
            } else {
                false
            }
        } else {
            false
        }
    };
    if !handled {
        screen.handle_message(message);
    }
}

fn app_logic(screen: &mut LiplScreen) -> impl WidgetView<LiplScreen> + use<> {
    fork(display(screen), task(background_task, on_message_received))
}

fn main() -> Result<(), EventLoopError> {
    tracing_subscriber::fmt::init();
    Xilem::new_simple(
        LiplScreen {
            text: String::default(),
            status: WAIT_MESSAGE.into(),
            font_size: DEFAULT_FONT_SIZE,
            dark: DEFAULT_DARK,
        },
        app_logic,
        WindowOptions::new(APP_TITLE),
    )
    .with_font(ROBOTO_FONT.to_vec())
    .run_in(EventLoop::with_user_event())
}
