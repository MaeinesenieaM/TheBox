//possible required code.
//use fastrand;

use std::fs;

use sdl2::pixels::Color;
//use sdl2::render::*;

use sdl2::event::Event;
use sdl2::keyboard::*;
//use sdl2::rect::*;

fn main() {
    println!("The box shall open, once again...");

    let sdl2 = sdl2::init().unwrap();
    let video_subsystem = sdl2.video().unwrap();

    let window = video_subsystem.window("Box", 800, 600)
        .position_centered()
        .build().unwrap();
    let mut canvas = window.into_canvas()
        .present_vsync()
        .build().unwrap();

    let mut event_pump = sdl2.event_pump().unwrap();

    match directory_verifier() {
        Ok (()) => {}
        Err (error) => { println! ("{:?}", error); }
    };

    'running: loop {
        canvas.set_draw_color(Color::RGB(20, 20, 20));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'running },
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => { println! ("TO DO!"); }
                _ => {}
            }
        }

        canvas.present();
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