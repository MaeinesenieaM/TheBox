//possible required code.
//use fastrand;
use std::fs;
//use std::io;

use sdl2::pixels::Color;
use sdl2::video::GLProfile;
//use sdl2::render::Canvas;

use sdl2::ttf;
//use sdl2::gfx;

use sdl2::event::Event;
use sdl2::keyboard::*;
//use sdl2::rect::*;


fn main() {
    println!("The box shall open, once again...");

    let sdl2 = sdl2::init().unwrap();
    let video_subsystem = sdl2.video().unwrap(); 

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 2);

    let window = video_subsystem.window("Box", 800, 600)
        .opengl()        
        .position_centered()
        .build().unwrap();

    let mut canvas = window.into_canvas()       
        .present_vsync()
        .build().unwrap();
    let texture_creator = canvas.texture_creator();   

    match directory_verifier() {
        Ok (()) => {}
        Err (error) => { println! ("{:?}", error); }
    };

    let ttf = ttf::init().unwrap();
    let ttf_font = ttf.load_font("./src/main_assets/Fixedsys.ttf", 32).expect("COULD NOT FIND FILE!");

    //creates the texture for text #TODO! Create a way to do this more intuitive.#
    let text_texture = ttf_font.render(":)").solid(Color::RGB(150, 150, 165)).unwrap().as_texture(&texture_creator).unwrap();

    let mut event_pump = sdl2.event_pump().unwrap();
    'running: loop {
        
        canvas.set_draw_color(Color::RGB(20, 20, 20));
        canvas.clear();
        let _ = canvas.copy(&text_texture, None, None);

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