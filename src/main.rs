//possible required code.
//use fastrand;

use std::fs;
//use std::io;

use sdl2::pixels::Color;

//use sdl2::render::Canvas;


//use sdl2::gfx;

use sdl2::event::Event;
use sdl2::keyboard::*;
//use sdl2::rect::*;

pub mod window;
use window::*;

fn main() {
    println!("The box shall open, once again...");

    let mut sdl_context = SdlContext::init_window(800, 600);
    let mut display = Display::init_display(sdl_context.window); 

    match directory_verifier() {
        Ok (()) => {}
        Err (error) => { println! ("{:?}", error); }
    };  

    let mut steps : i32 = 0;

    'running: loop {
        
        display.canvas.set_draw_color(Color::RGB(20, 20, 20));
        display.canvas.clear();
        display.create_text(0, (steps * 2) % 600, "i can do anything!", 16);
        steps += 1;

        for event in sdl_context.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'running },
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => { println! ("TO DO!"); }
                _ => {}
            }
        }

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