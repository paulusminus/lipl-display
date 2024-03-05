use std::error::Error;

use femtovg::{renderer::OpenGl, Canvas, Color, FontId, Paint};
use glutin::surface::GlSurface;
use lipl_display_common::{BackgroundThread, Command, HandleMessage, LiplScreen, Message};
use lipl_gatt_bluer::ListenBluer;
use log::error;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy},
    window::Window,
};

const ROBOTO_REGULAR: &[u8] = include_bytes!("../assets/Roboto-Regular.ttf");
const DEFAULT_FONT_SIZE: f32 = 32.0;
const BLACK: femtovg::Color = femtovg::Color::black();
const WHITE: femtovg::Color = femtovg::Color::white();

#[allow(dead_code)]
mod gatt_client;
mod helpers;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();

    let event_loop = EventLoopBuilder::<Message>::with_user_event().build()?;
    let (canvas, window, context, surface) = helpers::create_window("Text demo", &event_loop);
    run(canvas, event_loop, context, surface, window)
}

fn get_colors(dark: bool) -> (Color, Color) {
    if dark {
        (WHITE, BLACK)
    } else {
        (BLACK, WHITE)
    }
}

fn create_callback(proxy: EventLoopProxy<Message>) -> impl Fn(Message) {
    move |message| {
        if let Err(error) = proxy.send_event(message) {
            error!("Error sending to main loop: {}", error);
        }
    }
}

fn run(
    mut canvas: Canvas<OpenGl>,
    event_loop: EventLoop<Message>,
    context: glutin::context::PossiblyCurrentContext,
    surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
    window: Window,
) -> Result<(), Box<dyn Error>> {
    let proxy = event_loop.create_proxy();
    let mut gatt = ListenBluer::new(create_callback(proxy));

    let font_id = canvas.add_font_mem(ROBOTO_REGULAR)?;

    let mut screen = LiplScreen::new(true, DEFAULT_FONT_SIZE);

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

    event_loop.run(move |event, window_target| match event {
        Event::UserEvent(message) => {
            let mut exit = false;
            if let Message::Command(command) = &message {
                if command == &Command::Exit || command == &Command::Poweroff {
                    exit = true
                }
            }
            if exit {
                gatt.stop();
                window_target.exit();
            } else {
                screen.handle_message(message);
                window.request_redraw();
            }
        }
        Event::LoopExiting => {
            gatt.stop();
            window_target.exit();
        }
        Event::WindowEvent { ref event, .. } => match event {
            WindowEvent::Resized(physical_size) => {
                surface.resize(
                    &context,
                    physical_size.width.try_into().unwrap(),
                    physical_size.height.try_into().unwrap(),
                );
            }
            WindowEvent::CloseRequested => {
                gatt.stop();
                window_target.exit();
            }
            WindowEvent::RedrawRequested => {
                draw_paragraph(&mut canvas, font_id, &screen, &window);
                canvas.flush();
                surface.swap_buffers(&context).unwrap();
            }
            _ => (),
        },
        _ => (),
    })?;
    Ok(())
}

fn draw_paragraph(
    canvas: &mut Canvas<OpenGl>,
    font_id: FontId,
    part: &LiplScreen,
    window: &Window,
) {
    let dpi_factor = window.scale_factor();
    let size = window.inner_size();
    canvas.set_size(size.width, size.height, dpi_factor as f32);
    let x = 0.05 * canvas.width() as f32;
    let mut y = 0.05 * canvas.height() as f32 + part.font_size;

    let (fg_color, bg_color) = get_colors(part.dark);
    canvas.clear_rect(0, 0, size.width, size.height, bg_color);
    let mut paint = Paint::color(fg_color);
    paint.set_font(&[font_id]);
    paint.set_font_size(part.font_size);

    let font_metrics = canvas.measure_font(&paint).expect("Error measuring font");

    let width = canvas.width();

    let lines = canvas
        .break_text_vec(width as f32, &part.text, &paint)
        .expect("Error while breaking text");

    for line_range in lines {
        if let Ok(_res) = canvas.fill_text(x, y, &part.text[line_range], &paint) {
            y += font_metrics.height();
        }
    }

    y = canvas.height() as f32 - font_metrics.height();
    match canvas.fill_text(x, y, &part.status, &paint) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
