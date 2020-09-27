extern crate sdl2;

use std::thread::sleep;
use std::time::{Duration, SystemTime};

use sdl2::event::Event;
use sdl2::image::{INIT_JPG, INIT_PNG, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use tetrissdl::parse_int_array;
use std::fs::File;
use std::io;
use std::io::{Write, Read};

const TEXTURE_SIZE: u32 = 32;

#[derive(Debug)]
enum TextureColor {
    Green,
    Blue,
}

fn main() {
    {
        let values = parse_int_array!();
        println!("Tamanho {}", values.len());
    }


    let sdl_context = sdl2::init().expect("SDL initialization
      failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get
       SDL video subsystem");

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800,
                                        600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Failed to convert window into canvas");

    let texture_creator = canvas.texture_creator();

    sdl2::image::init(INIT_PNG | INIT_JPG).expect("Couldn't initialize
         image context");

    let image_texture =
        texture_creator.load_texture("assets/my_image.png")
            .expect("Couldn't load image");

    texture_creator.create_texture_target(None, TEXTURE_SIZE,
                                          TEXTURE_SIZE)
        .expect("Failed to create a texture");

    // We create a texture with a 32x32 size.
    let green_square = create_texture_rect(&mut canvas,
                                           &texture_creator,
                                           TextureColor::Green,
                                           TEXTURE_SIZE).expect("Failed to create a texture");

    let mut blue_square = create_texture_rect(&mut canvas,
                                              &texture_creator,
                                              TextureColor::Blue,
                                              TEXTURE_SIZE).expect("Failed to create a texture");
    let timer = SystemTime::now();

    let mut event_pump = sdl_context.event_pump().expect("Failed to
        get SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    {
                        break 'running;
                    }
                _ => {}
            }
        }

        // We set fulfill our window with red.
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        // We draw it.
        canvas.clear();

        canvas.copy(&image_texture, None, None)
            .expect("Render failed");


        let display_green = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() % 2 == 0,
            Err(_) => {
                // In case of error, we do nothing...
                true
            }
        };
        let square_texture = if display_green {
            &green_square
        } else {
            &blue_square
        };
        // Copy our texture into the window.
        canvas.copy(square_texture,
                    None,
                    // We copy it at the top-left of the window with a 32x32
                    Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE))
            .expect("Couldn't copy texture into window");
        // We update window's display.
        canvas.present();

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn create_texture_rect<'a>(canvas: &mut Canvas<Window>,
                           texture_creator: &'a TextureCreator<WindowContext>,
                           color: TextureColor, size: u32) -> Option<Texture<'a>> {
    // We'll want to handle failures outside of this function.
    if let Ok(mut square_texture) =
    texture_creator.create_texture_target(None, size, size) {
        canvas.with_texture_canvas(&mut square_texture, |texture| {
            match color {
                TextureColor::Green =>
                    texture.set_draw_color(Color::RGB(0, 255, 0)),
                TextureColor::Blue =>
                    texture.set_draw_color(Color::RGB(0, 0, 255)),
            }
            texture.clear();
        }).expect("Failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
}

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = File::create(file_name)?;
    f.write_all(content.as_bytes())
}

fn read_from_file(file_name: &str) -> io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

