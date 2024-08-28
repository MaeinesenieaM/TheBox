use sdl2::event::Event;
use sdl2::rect::*;
use sdl2::pixels::Color;

use sdl2::keyboard::*;
use sdl2::mouse::*;

use thebox::*;

//use sdl2::gfx::primitives::DrawRenderer;
/*
struct Entity {
    fill: bool,
    data: i32,
    pos_x: i16,
    pos_y: i16,
}
*/

use thebox::{Display, Write};

pub const NAME: &str = "Order";
pub const ID: u8 = 9;

pub fn start(display: &mut Display, event_pump: &mut sdl2::EventPump, write: &mut Write) {
	let window = display.canvas.window();

	let points = grid_points(window.size(), 16);

    'repeat: loop {
        let _mouse = MouseState::new(&event_pump);

    	display.canvas.set_draw_color(DEFAULT_CLEAR_COLOR);
        display.canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | 
                Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => break 'repeat,
                _ => {}
            }
        }

        let mut count = 0;

        for point in points.iter() {
            count = count + 1;
        	let pos: (i32, i32) = sdl2::rect::Point::from(*point.clone()).into();
            let rectangle = Rect::new(pos.0, pos.1, 16, 16);
            display.canvas.set_draw_color(Color::RGB(100, 120, 100));
            let _ = display.canvas.draw_rect(rectangle);
            display.canvas.set_draw_color(Color::RGB(200, 200, 200));
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
	let width: i32 = i32::try_from(size.0).unwrap();
	let height: i32 = i32::try_from(size.1).unwrap();

	let ammount_x: i32 = width / difference;
	let ammount_y: i32 = height / difference;

	let mut points: Vec<Point> = Vec::new();

	for i in 0..ammount_y {
		for j in 0..ammount_x {
			points.push(Point::new((width / ammount_x) * j, (height / ammount_y) * i));
		}
	}

	points
}