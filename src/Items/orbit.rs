use sdl2::event::Event;
use sdl2::keyboard::*;
use sdl2::pixels::Color;

use sdl2::gfx::primitives::DrawRenderer;

use thebox::{Display, Write};

pub const NAME: &str = "Orbit";
pub const ID: u8 = 1;

pub fn start(display: &mut Display, event_pump: &mut sdl2::EventPump, write: &mut Write) {
    let radius: i16 = 100;

    let mut blue: u8 = 120;
    let mut over: bool = false;

    let mut angle: f32 = 0.0;


    let windowref = display.canvas.window();

    let (window_x, window_y) = windowref.size();
    let window_x_middle: i16 = (window_x / 2).try_into().unwrap();
    let window_y_middle: i16 = (window_y / 2).try_into().unwrap();

    let circle_color: Color = Color::RGB(120, 120, 120);

    'repeat: loop {
        let circle_sin_x: i16 = window_y_middle + (100.0 * angle.sin()) as i16;
        let circle_cos_y: i16 = window_x_middle + (100.0 * angle.cos()) as i16;

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

        display.canvas.set_draw_color(Color::RGB(20, 20, 20));
        display.canvas.clear();

        write.set_draw_color_rgb(120, 120, blue);
        display.draw_text_centered(&write, window_x_middle as i32, (window_y_middle + 100) as i32, "ORBIT!", 16);

        let _ = display.canvas.aa_circle(window_x_middle, window_y_middle, radius, circle_color);
        let _ = display.canvas.filled_circle(circle_cos_y, circle_sin_x, radius / 8, circle_color);

        if blue == 255 || blue == 0 {
            over = !over
        };

        if over == true {
            blue += 1
        } else {
            blue -= 1
        };

        angle += 0.02;

        display.draw_text(
            &write,
            0,
            0,
            &angle.to_string(),
            8,
        );

        display.canvas.present();
    }

    write.set_draw_color(super::super::DEFAULT_COLOR);
}