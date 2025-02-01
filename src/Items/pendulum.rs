//use sdl2::pixels::Color;
use sdl2::keyboard::*;
use sdl2::rect::*;

use thebox::*;

pub const NAME: &str = "Double Pendulum";
pub const ID: u8 = 3;

struct Pendulum {
    pos: FPoint,
    angle: f32,
    length: f32,
    velocity_x: f32,
    velocity_y: f32
}

impl Pendulum {
    pub fn update(&mut self) {
        
    }
}

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &Write) {
    
    let message: Label = Label::new(
            400,
            550,
            16,
            &write,
            Some(String::from("Will simulate a double pendulum."))
    );
    
    let mut gravity: f32 = 50.0;
    
    'repeat: loop {
        display.canvas.clear();

        let keyboard: KeyboardState = KeyboardState::new(&sdl_context.event_pump);
        
        if keyboard.is_scancode_pressed(Scancode::Escape) {let _ = sdl_context.send_quit();}
        if sdl_context.check_quit() {break 'repeat}

        let _ = message.draw(display);

        display.canvas.present();
    }
}