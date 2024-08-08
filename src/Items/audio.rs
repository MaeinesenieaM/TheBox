use sdl2::event::Event;
//use sdl2::pixels::Color;
use sdl2::keyboard::*;

//use sdl2::gfx::primitives::DrawRenderer;

use thebox::{Display, Write};

pub const NAME: &str = "Audio";
pub const ID: u8 = 5;

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
            "There will be audio playing here at some point.",
            8,
        );

        display.canvas.present();
    }
}
