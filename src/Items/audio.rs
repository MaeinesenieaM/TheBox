use sdl2::event::Event;
//use sdl2::pixels::Color;
use sdl2::keyboard::*;

use thebox::{Display, Write, SdlContext};

pub const NAME: &str = "Audio";
pub const ID: u8 = 6;

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &mut Write) {

//   let audio_path: Path = 

    'repeat: loop {
        display.canvas.clear();

        for event in sdl_context.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'repeat,
                _ => {}
            }
        }

        display.draw_text_centered(
            &write,
            400,
            550,
            "There will be audio playing here at some point.",
            8,
        );

        display.canvas.present();
    }
}
