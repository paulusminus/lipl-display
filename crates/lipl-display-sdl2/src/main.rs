use std::error::Error;
use std::time::Duration;

use lipl_display_common::{Part, Message, HandleMessage};
use sdl2::VideoSubsystem;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

const FONT_FILE: &str = "/usr/share/fonts/google-roboto/Roboto-Regular.ttf";

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

const BLACK: Color = Color::RGBA(0, 0, 0, 255);
const WHITE: Color = Color::RGBA(255, 255, 255, 255);

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn create_draw(video_subsys: VideoSubsystem) -> Result<impl FnMut(Message), Box<dyn Error>> {
    let mut part = Part::new(true, String::new(), 18.0);

    let ttf_context = sdl2::ttf::init()?;
    let window = video_subsys
        .window("SDL2_TTF Example", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    let texture_creator = canvas.texture_creator();

    // Load a font
    let font = ttf_context.load_font(FONT_FILE, 25)?;

    // render a surface, and convert it to a texture bound to the canvas
    let surface = font
        .render("Hello Rust!")
        .blended(BLACK)?;

    let texture = texture_creator
        .create_texture_from_surface(&surface)?;

    canvas.set_draw_color(WHITE);
    canvas.clear();

    let TextureQuery { width, height, .. } = texture.query();

    // If the example text is too big for the screen, downscale it (and center irregardless)
    let padding = 64;
    let target = get_centered_rect(
        width,
        height,
        SCREEN_WIDTH - padding,
        SCREEN_HEIGHT - padding,
    );
    
    canvas.copy(&texture, None, Some(target))?;
    canvas.present();
    
    Ok(
        move |message| {
            part = part.handle_message(message);
        }
    )
}


// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

fn run() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let event_subsystem = sdl_context.event()?;
    event_subsystem.register_custom_event::<Message>()?;
    let event_sender = event_subsystem.event_sender();

    let gatt = lipl_gatt_bluer::ListenBluer::new(move |message| {
        if let Err(error) = event_sender.push_custom_event(message) {
            eprintln!("Error: {}", error);
        }
    });


    let mut draw = create_draw(sdl_context.video()?)?;

    for event in sdl_context.event_pump()?.wait_iter() {
        if event.is_user_event() {
            if let Some(message) = event.as_user_event_type::<Message>() {
                draw(message);
            }
        }
        else {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break,
                _ => {}
            }
        }
    };

    drop(gatt);
    std::thread::sleep(Duration::from_secs(1));

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("linked sdl2_ttf: {}", sdl2::ttf::get_linked_version());
    run()?;
    Ok(())
}
