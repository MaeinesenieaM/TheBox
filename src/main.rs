//possible required code.
//use fastrand;

use std::fs;
use std::time::{Duration, Instant};
//use std::io;

use sdl2::pixels::Color;

//use sdl2::render::Canvas;
//use sdl2::gfx;

use sdl2::event::Event;
use sdl2::keyboard::*;
//use sdl2::rect::*;

pub mod window;
pub mod items;
use window::*;

fn main() {
    println!("The box shall open, once again...");

    let mut sdl_context = SdlContext::init_context();
    let mut display = Display::init_display(&sdl_context.video_subsystem, 800, 600); 

    match directory_verifier() {
        Ok (()) => {}
        Err (error) => { println! ("{:?}", error); }
    };  

    let mut frames = 0;
    let mut temp_frames = 0;
    let mut instant = Instant::now();

    let mut count : i32 = 0;
    let mut enter : u16 = 0;

    'running: loop {
        
        display.canvas.set_draw_color(Color::RGB(20, 20, 20));

        for event in sdl_context.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'running },
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => { enter = 1; },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => { count -= 1; },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => { count += 1; },
                _ => {}
            }
        }

        if enter == 1 {
            items::select_item(count, &mut display, &mut sdl_context.event_pump);
            enter = 0;
        }

        //basic framerate counter.
        if instant.elapsed() >= Duration::from_secs(1) {
            temp_frames = frames;
            frames = 0;
            instant = Instant::now();
        }

        
        display.canvas.clear();

        display.create_text_centered(400, 300, &count.to_string(), 16);
        display.create_text(0, 0, &temp_frames.to_string(), 8);

        frames += 1;
        display.canvas.present();
    }
}

fn directory_verifier () -> std::io::Result<()> {

    for entry in fs::read_dir("./src")? {
        let entry = entry?;
        if entry.path().to_str().unwrap() == "./src\\Items" { 
            println! ("Found [Items] directory!");
            return Ok (());
        }        
    }
    println! ("[Items] directory not found.\nCreating [Items] directory...");
    fs::create_dir("./src/Items")?;
    println! ("[Items] created!");

    Ok (())
}