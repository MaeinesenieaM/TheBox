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

    let mut sdl_context = SdlContext::init_window(800, 600);
    let mut display = Display::init_display(sdl_context.window); 

    match directory_verifier() {
        Ok (()) => {}
        Err (error) => { println! ("{:?}", error); }
    };  

    let mut frames = 0;
    let mut temp_frames = 0;
    let mut instant = Instant::now();

    let mut conta : i32 = 0;

    'running: loop {
        
        display.canvas.set_draw_color(Color::RGB(20, 20, 20));
        display.canvas.clear();

        for event in sdl_context.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'running },
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => { items::select_item(conta); },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => { conta -= 1; },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => { conta += 1; },
                _ => {}
            }
        }

        //basic framerate counter.
        if instant.elapsed() >= Duration::from_secs(1) {
            temp_frames = frames;
            frames = 0;
            instant = Instant::now();
        }

        display.create_text(400, 300, &conta.to_string(), 16);
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