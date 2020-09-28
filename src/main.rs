extern crate rand;
extern crate sdl2;

use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use sdl2::event::Event;
use sdl2::image::{INIT_JPG, INIT_PNG, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use tetris::{LEVEL_LINES, LEVEL_TIMES, Tetris};

use crate::tetrimino::Tetrimino;

mod tetris;
mod tetrimino;
mod game_map;

// use tetrissdl::parse_int_array;

const TEXTURE_SIZE: u32 = 32;


#[derive(Debug)]
enum TextureColor {
    Green,
    Blue,
}

fn main() {
    /*    {
            let values = parse_int_array!();
            println!("Tamanho {}", values.len());
        }*/


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

    let blue_square = create_texture_rect(&mut canvas,
                                              &texture_creator,
                                              TextureColor::Blue,
                                              TEXTURE_SIZE).expect("Failed to create a texture");
    let mut tetris = Tetris::new();

    let mut timer = SystemTime::now();

    let mut event_pump = sdl_context.event_pump().expect("Failed to
        get SDL event pump");

/*    'running: loop {
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
    }*/

    loop {
        if is_time_over(&tetris, &timer) {
            let mut make_permanent = false;
            if let Some(ref mut piece) = tetris.get_current_piece() {
                let x = piece.get_x();
                let y = piece.get_y() + 1;
                make_permanent = !tetris.change_current_piece_position(x, y);
            }
            if make_permanent {
                tetris.make_permanent();
            }
            timer = SystemTime::now();
        }

        // We need to draw the tetris "grid" in here.

        if tetris.get_current_piece().is_none() {
            let current_piece = Tetrimino::create_new_tetrimino();
            if !current_piece.test_current_position(&tetris.game_map) {
                print_game_information(&tetris);
                break;
            }
            tetris.set_current_piece(current_piece);
        }

        let mut quit = false;
        if !handle_events(&mut tetris, &mut quit, &mut timer, &mut event_pump) {
            if let Some(mut piece) = tetris.get_current_piece() {
                // We need to draw our current tetrimino here

            }
        }
        if quit {
            print_game_information(&tetris);
            break
        }

        // TODO: we need to draw the game map here.

        sleep(Duration::new(0, 1_000_000u32 / 60));
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

fn slice_to_string(slice: &[u32]) -> String {
    slice.into_iter()
        .map(|fatia| fatia.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn line_to_slice(line: &str) -> Vec<u32> {
    line
        .split(" ")
        .filter_map(|nb| nb.parse::<u32>().ok())
        .collect::<Vec<u32>>()
}

fn save_highscores_and_lines(highscores: &[u32], number_of_lines: &[u32]) -> bool {
    let s_highscores = slice_to_string(highscores);
    let s_number_of_lines = slice_to_string(number_of_lines);

    write_into_file(
        &*format!("{}\n{}\n", s_highscores, s_number_of_lines),
        "scores.txt")
        .is_ok()
}

fn load_highscores_and_lines() -> Option<(Vec<u32>, Vec<u32>)> {
    if let Ok(content) = read_from_file("scores.txt") {
        let mut lines = content
            .splitn(2, "\n")
            .map(|line| line_to_slice(line))
            .collect::<Vec<_>>();
        if lines.len() == 2 {
            let (number_lines, highscores) =
                (lines.pop().unwrap(), lines.pop().unwrap());
            Some((number_lines, highscores))
        } else {
            None
        }
    } else {
        None
    }
}

fn handle_events(tetris: &mut Tetris, quit: &mut bool, timer: &mut SystemTime, event_pump: &mut sdl2::EventPump)
                 -> bool {
    let mut make_permanent = false;
    if let Some(_) = tetris.get_current_piece() {
        let mut tmp_x = tetris.get_current_piece().unwrap().get_x();
        let mut tmp_y = tetris.get_current_piece().unwrap().get_y();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    *quit = true;
                    break;
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    *timer = SystemTime::now();
                    tmp_y += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    tmp_x += 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    tmp_x += -1;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    tetris.rotate_current_piece();
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    let x = tetris.get_current_piece().unwrap().get_x();
                    let mut y = tetris.get_current_piece().unwrap().get_y();
                    while tetris.change_current_piece_position(x, y + 1) == true {
                        y += 1;
                    }
                    make_permanent = true;
                }
                _ => {}
            }
        }

        if !make_permanent {
            if tetris.change_current_piece_position( tmp_x, tmp_y) == false
                && tmp_y != tetris.get_current_piece().unwrap().get_y() {
                make_permanent = true;
            }
        }
        if make_permanent {
            tetris.make_permanent();
            *timer = SystemTime::now();
        }
    }
    make_permanent
}

fn print_game_information(tetris: &Tetris) {
    println!("Game over...");
    println!("Score:           {}", tetris.get_score());
    // println!("Number of lines: {}", tetris.nb_lines);
    println!("Current level:   {}", tetris.get_current_level());
    // Check highscores here and update if needed
}

fn is_time_over(tetris: &Tetris, timer: &SystemTime) -> bool {
    match timer.elapsed() {
        Ok(elapsed) => {
            let millis = elapsed.as_secs() as u32 * 1000 +
                elapsed.subsec_nanos() / 1_000_000;
            millis > LEVEL_TIMES[tetris.get_current_level() as usize - 1]
        }
        Err(_) => false,
    }
}
