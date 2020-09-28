extern crate rand;
extern crate sdl2;

use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use sdl2::event::Event;
// use sdl2::image::{INIT_JPG, INIT_PNG, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use tetris::{Tetris};

use crate::tetrimino::Tetrimino;
use crate::tetris::LEVEL_TIMES;

mod tetris;
mod tetrimino;
mod game_map;

// use tetrissdl::parse_int_array;

//const TEXTURE_SIZE: u32 = 32;

const TETRIS_HEIGHT: usize = 40;
const HIGHSCORE_FILE: &'static str = "scores.txt";


#[derive(Debug)]
enum TextureColor {
    Green,
    Blue,
    Red,
    Black,
}

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");

    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");
    let width = 600;
    let height = 800;

    let mut tetris = Tetris::new();
    let mut timer = SystemTime::now();

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    // let grid_x = (width - TETRIS_HEIGHT as u32 * 10) as i32 / 2;
    let grid_x = 20;
    let grid_y = (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2;

    let window = video_subsystem.window("Tetris", width, height)
        .position_centered() // to put it in the middle of the screen
        .build() // to create the window
        .expect("Failed to create window");

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync() // To enable v-sync.
        .build()
        .expect("Couldn't get window's canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let grid = create_texture_rect(&mut canvas, &texture_creator, TextureColor::Black, TETRIS_HEIGHT as u32 * 10).expect("Failed to create a texture");

    let border = create_texture_rect(&mut canvas, &texture_creator, TextureColor::Blue, TETRIS_HEIGHT as u32 * 10 + 20).expect("Failed to create a texture");

    macro_rules! texture {
        ($r:expr, $g:expr, $b:expr) => (
            create_texture_rect(&mut canvas, &texture_creator, TextureColor::Green, TETRIS_HEIGHT as u32).unwrap()
        )
      }

    let textures = [texture!(255, 69, 69), texture!(255, 220, 69),
        texture!(237, 150, 37), texture!(171, 99, 237), texture!(77, 149,
        239), texture!(39, 218, 225), texture!(45, 216, 47)];

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

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        // Draw the tetris "grid" in here.
        canvas.copy(&border,
                    None,
                    Rect::new(10,
                              (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2 - 10,
                              TETRIS_HEIGHT as u32 * 10 + 20, TETRIS_HEIGHT as u32 * 16 + 20))
            .expect("Couldn't copy texture into window");
        canvas.copy(&grid,
                    None,
                    Rect::new(20,
                              (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2,
                              TETRIS_HEIGHT as u32 * 10, TETRIS_HEIGHT as u32 * 16))
            .expect("Couldn't copy texture into window");

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
                for (line_nb, line) in piece.states[piece.get_current_state() as usize].iter().enumerate() {
                    for (case_nb, case) in line.iter().enumerate() {
                        if *case == 0 {
                            continue;
                        }
                        // The new part is here:
                        canvas.copy(&textures[*case as usize - 1],
                                    None,
                                    Rect::new(grid_x + (piece.get_x() + case_nb as isize) as
                                        i32 * TETRIS_HEIGHT as i32, grid_y + (piece.get_y() +
                                        line_nb) as i32 * TETRIS_HEIGHT as i32, TETRIS_HEIGHT
                                                  as u32, TETRIS_HEIGHT as u32))
                            .expect("Couldn't copy texture into window");
                    }
                }
            }
        }
        if quit {
            print_game_information(&tetris);
            break;
        }

        let ttf_context = sdl2::ttf::init().expect("SDL TTF initialization
        failed");
        let mut font = ttf_context.load_font("assets/lucon.ttf", 128).expect("Couldn't load the font");
        font.set_style(sdl2::ttf::STYLE_BOLD);

        let rendered_text = create_texture_from_text(&texture_creator,
                                                     &font, "test", 0, 0, 0).expect("Cannot render text");
        canvas.copy(&rendered_text, None, Some(Rect::new(width as i32 -
                                                             40, 0, 40, 30))).expect("Couldn't copy text");
        // display_game_information(&tetris, &mut canvas, &texture_creator, &font,
        //                          width as i32 - grid_x - 10);

        // Draw the game map here.
        for (line_nb, line) in tetris.game_map.iter().enumerate() {
            for (case_nb, case) in line.iter().enumerate() {
                if *case == 0 {
                    continue;
                }
                canvas.copy(&textures[*case as usize - 1],
                            None, Rect::new(grid_x + case_nb as i32 * TETRIS_HEIGHT
                        as i32, grid_y + line_nb as i32 * TETRIS_HEIGHT as i32,
                                            TETRIS_HEIGHT as u32, TETRIS_HEIGHT as u32))
                    .expect("Couldn't copy texture into window");
            }
        }

        canvas.present();

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
                TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
                TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)),
                TextureColor::Red => texture.set_draw_color(Color::RGB(255, 0, 0)),
                TextureColor::Black => texture.set_draw_color(Color::RGB(0, 0, 0)),
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
            if tetris.change_current_piece_position(tmp_x, tmp_y) == false
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
    let mut new_highest_highscore = true;
    let mut new_highest_lines_sent = true;
    if let Some((mut highscores, mut lines_sent)) =
    load_highscores_and_lines() {
        new_highest_highscore = update_vec(&mut highscores,
                                           tetris.score);
        new_highest_lines_sent = update_vec(&mut lines_sent,
                                            tetris.nb_lines);
        if new_highest_highscore || new_highest_lines_sent {
            save_highscores_and_lines(&highscores, &lines_sent);
        }
    } else {
        save_highscores_and_lines(&[tetris.score], &
            [tetris.nb_lines]);
    }
    println!("Game over...");
    println!("Score:           {}{}",
             tetris.score,
             if new_highest_highscore { " [NEW HIGHSCORE]" } else {
                 ""
             });
    println!("Number of lines: {}{}",
             tetris.nb_lines,
             if new_highest_lines_sent { " [NEW HIGHSCORE]" } else {
                 ""
             });
    println!("Current level:   {}", tetris.get_current_level());
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

const NB_HIGHSCORES: usize = 5;

fn update_vec(v: &mut Vec<u32>, value: u32) -> bool {
    if v.len() < NB_HIGHSCORES {
        v.push(value);
        v.sort();
        true
    } else {
        for entry in v.iter_mut() {
            if value > *entry {
                *entry = value;
                return true;
            }
        }
        false
    }
}

fn create_texture_from_text<'a>(texture_creator: &'a TextureCreator<WindowContext>,
                                font: &sdl2::ttf::Font,
                                text: &str,
                                r: u8, g: u8, b: u8,
) -> Option<Texture<'a>> {
    if let Ok(surface) = font.render(text)
        .blended(Color::RGB(r, g, b)) {
        texture_creator.create_texture_from_surface(&surface).ok()
    } else {
        None
    }
}

fn get_rect_from_text(text: &str, x: i32, y: i32) -> Option<Rect> {
    Some(Rect::new(x, y, text.len() as u32 * 20, 30))
}