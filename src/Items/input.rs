use sdl2::event::Event;
//use sdl2::pixels::Color;
use sdl2::keyboard::*;

use thebox::{Display, Write};

pub const NAME: &str = "Input";
pub const ID: u8 = 8;

pub fn start(display: &mut Display, event_pump: &mut sdl2::EventPump, write: &mut Write) {
    'repeat: loop {
        display.canvas.clear();

        for event in event_pump.poll_iter() {
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
            "SLIDERS! This will be an example of some user input tools.",
            8,
        );

        display.canvas.present();
    }
}
