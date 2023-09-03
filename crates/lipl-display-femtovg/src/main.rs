use std::error::Error;

use femtovg::{renderer::OpenGl, Canvas, Color, FontId, Paint};
use glutin::surface::GlSurface;
use lipl_display_common::{Command, Part, HandleMessage, Listen, Message};
use lipl_gatt_bluer::ListenBluer;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopBuilder},
    window::Window,
};

const ROBOTO_REGULAR: &[u8] = include_bytes!("../assets/Roboto-Regular.ttf");
const DEFAULT_FONT_SIZE: f32 = 32.0;
const BLACK: femtovg::Color = femtovg::Color::black();
const WHITE: femtovg::Color = femtovg::Color::white();

mod helpers;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new().init()?;
    log::set_max_level(log::LevelFilter::Info);

    let event_loop = EventLoopBuilder::<Message>::with_user_event().build();
    let (canvas, window, context, surface) = helpers::create_window("Text demo", &event_loop);
    run(canvas, event_loop, context, surface, window)
}


fn get_colors(dark: bool) -> (Color, Color) {
    if dark {
        (WHITE, BLACK)
    }
    else {
        (BLACK, WHITE)
    }
}

fn run(
    mut canvas: Canvas<OpenGl>,
    el: EventLoop<Message>,
    context: glutin::context::PossiblyCurrentContext,
    surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
    window: Window,
) -> Result<(), Box<dyn Error>> {
    let proxy = el.create_proxy();
    let mut gatt = ListenBluer { sender: None };
    gatt.listen_background(move |message| {
        if let Err(error) = proxy.send_event(message) {
            log::error!("Error sending to main loop: {}", error);
        }
    });

    let font_id = canvas.add_font_mem(ROBOTO_REGULAR)?;

    let mut part = Part::new(true, "Even geduld a.u.b. ...".to_owned(), DEFAULT_FONT_SIZE);

    el.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::UserEvent(message) => {
                let mut exit = false;
                if let Message::Command(command) = &message {
                    if command == &Command::Exit || command == &Command::Poweroff {
                        exit = true
                    }
                }
                if exit { 
                    gatt.stop();
                    control_flow.set_exit();
                } 
                else { 
                    part = part.handle_message(message);
                    window.request_redraw();
                }
            },
            Event::LoopDestroyed => {
                gatt.stop();
                control_flow.set_exit();
            },
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
                    control_flow.set_exit();
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                draw_paragraph(&mut canvas, font_id, &part, &window);
                canvas.flush();
                surface.swap_buffers(&context).unwrap();
            },
            Event::MainEventsCleared => window.request_redraw(),
            _ => (),
        }
    });
}

fn draw_paragraph(canvas: &mut Canvas<OpenGl>, font_id: FontId, part: &Part, window: &Window) {
    let dpi_factor = window.scale_factor();
    let size = window.inner_size();
    canvas.set_size(size.width, size.height, dpi_factor as f32);
    let x = 0.05 * canvas.width();
    let mut y = 0.05 * canvas.height() + part.font_size;

    let (fg_color, bg_color) = get_colors(part.dark);
    canvas.clear_rect(0, 0, size.width, size.height, bg_color);
    let mut paint = Paint::color(fg_color);
    paint.set_font(&[font_id]);
    paint.set_font_size(part.font_size);

    let font_metrics = canvas.measure_font(&paint).expect("Error measuring font");

    let width = canvas.width();

    let lines = canvas
        .break_text_vec(width, &part.text, &paint)
        .expect("Error while breaking text");

    for line_range in lines {
        if let Ok(_res) = canvas.fill_text(x, y, &part.text[line_range], &paint) {
            y += font_metrics.height();
        }
    }

    y = canvas.height() - font_metrics.height(); 
    match canvas.fill_text(x, y, &part.status, &paint) {
        Ok(_) => {},
        Err(e) => { eprintln!("Error: {}", e); }
    }
}
