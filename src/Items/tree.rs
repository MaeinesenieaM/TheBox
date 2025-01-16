use sdl2::event::Event;
//use sdl2::pixels::Color;
use sdl2::keyboard::*;

use thebox::{Display, Write, SdlContext};

pub const NAME: &str = "Tree";
pub const ID: u8 = 4;

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &mut Write) {
    'repeat: loop {
        display.canvas.clear();

        let keyboard: KeyboardState = KeyboardState::new(&sdl_context.event_pump);
        
        if keyboard.is_scancode_pressed(Scancode::Escape) {let _ = sdl_context.send_quit();}
        if sdl_context.check_quit() {break 'repeat}

        display.draw_text_centered(
            &write,
            400,
            550,
            "There will be procedural a tree in here.",
            16,
        );

        display.canvas.present();
    }
}
