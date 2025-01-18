//use sdl2::pixels::Color;
use sdl2::keyboard::*;
//use sdl2::rect::*;

use thebox::*;

pub const NAME: &str = "Double Pendulum";
pub const ID: u8 = 3;

/*
struct Pendulum {
    center: Point,
    center_ange: f32,
    p1: Point,
    p1_distance: f32,
    p1_angle: f32,
    p2: Point,
    p2_distance: f32,
    p2_angle: f32,
}
*/

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &Write) {
    
    let message: Label = Label::new(
            400,
            550,
            16,
            &write,
            Some(String::from("Will simulate a double pendulum."))
    );
    
    'repeat: loop {
        display.canvas.clear();

        let keyboard: KeyboardState = KeyboardState::new(&sdl_context.event_pump);
        
        if keyboard.is_scancode_pressed(Scancode::Escape) {let _ = sdl_context.send_quit();}
        if sdl_context.check_quit() {break 'repeat}

        let _ = message.draw_centered(display);

        display.canvas.present();
    }
}
