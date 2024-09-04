use sdl2::event::Event;
//use sdl2::pixels::Color;
use sdl2::keyboard::*;

use thebox::{Display, Write};

pub const NAME: &str = "Double Pendulum";
pub const ID: u8 = 3;

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

        display.draw_text_centered(&write, 400, 550, "Will simulate a double pendulum.", 16);

        display.canvas.present();
    }
}
