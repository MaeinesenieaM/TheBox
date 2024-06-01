use sdl2::event::Event;
use sdl2::rect::*;
use sdl2::pixels::Color;
use sdl2::keyboard::*;

//use sdl2::gfx::primitives::DrawRenderer;

use crate::window::{Display, Write};

pub const NAME : &str = "Order";
pub const ID : u8 = 3;

pub fn start(display: &mut Display, event_pump: &mut sdl2::EventPump, write: &mut Write) {

	let window = display.canvas.window();

	let points = grid_points(window.size(), 16);
    'repeat: loop {
    	display.canvas.set_draw_color(super::super::DEFAULT_CLEAR_COLOR);
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

        for point in points.iter() {
        	display.canvas.set_draw_color(Color::RGB(100, 100, 100));
        	let _ = display.canvas.draw_point(*point);
        }

        display.draw_text_centered(
            &write,
            400,
            550,
            "This section will have a vector visualizer and it will order it.",
            8,
        );

        display.canvas.present();
    }
}

//Calculates the grid and return the Points.
fn grid_points(size: (u32, u32), difference: i32) -> Vec<Point> {
	let width : i32 = i32::try_from(size.0).unwrap();
	let height : i32 = i32::try_from(size.1).unwrap();

	let ammount_x : i32 = width / difference;
	let ammount_y : i32 = height / difference;

	let mut points : Vec<Point> = Vec::new();

	for i in 0..ammount_y {
		for j in 0..ammount_x {
			points.push(Point::new((width / ammount_x) * j, (height / ammount_y) * i));
		}
	}

	points
}