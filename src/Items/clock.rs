use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::*;

use sdl2::gfx::primitives::DrawRenderer;

use crate::window::{Display, Write};

pub const NAME : &str = "Clock";
pub const ID : u8 = 6;

pub fn start(display: &mut Display, event_pump: &mut sdl2::EventPump, write: &mut Write) {

    let (window_x, window_y) : (u32, u32) = display.canvas.window().size();

    let window_x : i16 = window_x.try_into().unwrap();
    let window_y : i16 = window_y.try_into().unwrap();

    'repeat: loop {
        display.canvas.set_draw_color(Color::RGB(20, 20, 20));
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

        let _ = display.canvas.circle(window_x / 2, window_y / 2, 200, Color::WHITE);

        display.draw_text_centered(
            &write,
            400,
            550,
            "In here, there will be a analog clock that tells time live.",
            8,
        );

        display.canvas.present();
    }
}
