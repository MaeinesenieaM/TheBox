//possible required code.
//use fastrand;
use std::time::{Duration, Instant};

//use sdl2::pixels::Color;

use sdl2::event::Event;
use sdl2::keyboard::*;

use thebox::*;

pub mod items;

fn main() {
    println!("The box shall open, once again...");

    let mut sdl_context = SdlContext::init_context();
    let mut display = Display::init_display(&sdl_context.video_subsystem, 800, 600);

    let ttf = sdl2::ttf::init().unwrap();
    let mut write = Write::init_write(&ttf, DEFAULT_COLOR, "Fixedsys.ttf");

    let mut frames = 0;
    let mut temp_frames = 0;
    let mut instant = Instant::now();

    let mut count: i32 = 1;
    let mut enter: u16 = 0;

    'running: loop {
        for event in sdl_context.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
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
            items::select_item(count, &mut display, &mut sdl_context, &mut write);
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
