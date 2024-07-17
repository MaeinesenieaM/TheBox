//possible required code.
//use fastrand;

use std::fs;
use std::time::{Duration, Instant};

use sdl2::pixels::Color;

use sdl2::event::Event;
use sdl2::keyboard::*;

pub mod items;
pub mod window;
use window::*;

pub const DEFAULT_COLOR: Color = Color::RGB(210, 210, 220);
pub const DEFAULT_CLEAR_COLOR: Color = Color::RGB(20, 20, 20);

fn main() {
    println!("The box shall open, once again...");

    let mut sdl_context = SdlContext::init_context();
    let mut display = Display::init_display(&sdl_context.video_subsystem, 800, 600);

    match directory_verifier() {
        Ok(()) => {}
        Err(error) => {
            println!("{:?}", error);
        }
    };

    let ttf = sdl2::ttf::init().unwrap();
    let mut write = Write::init_write(&ttf, DEFAULT_COLOR);

    let mut frames = 0;
    let mut temp_frames = 0;
    let mut instant = Instant::now();

    let mut count: i32 = 1;
    let mut enter: u16 = 0;

    'running: loop {
        for event in sdl_context.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    enter = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    count -= 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    count += 1;
                }
                _ => {}
            }
        }

        if enter == 1 {
            items::select_item(count, &mut display, &mut sdl_context.event_pump, &mut write);
            enter = 0;
        }

        //basic framerate counter.
        if instant.elapsed() >= Duration::from_secs(1) {
            temp_frames = frames;
            frames = 0;
            instant = Instant::now();
        }

        display.canvas.set_draw_color(DEFAULT_CLEAR_COLOR);
        display.canvas.clear();

        display.draw_text_centered(&write, 400, 300, items::name_item(count), 16);
        display.draw_text_centered(&write, 400, 332, &count.to_string(), 8);
        display.draw_text(&write, 0, 0, &temp_frames.to_string(), 8);

        frames += 1;
        display.canvas.present();
    }
}

fn directory_verifier() -> std::io::Result<()> {
    for entry in fs::read_dir("./src")? {
        let entry = entry?;
        if entry.path().to_str().unwrap() == "./src\\Items" {
            println!("Found [Items] directory!");
            return Ok(());
        }
    }
    println!("[Items] directory not found.\nCreating [Items] directory...");
    fs::create_dir("./src/Items")?;
    println!("[Items] created!");

    Ok(())
}
