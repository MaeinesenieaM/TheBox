use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::*;

use sdl2::gfx::primitives::DrawRenderer;

use crate::window::{Display, Write};

pub fn start (display: &mut Display, event_pump: &mut sdl2::EventPump, write: &mut Write) {
	
	let radius : i16 = 100;

	let mut angle : f32 = 0.0;
	
	let circle_color : Color = Color::RGB(120, 120, 120);

	'repeat: loop {

		let circle_cos_y : i16 = unsafe { (400.0 + (100.0 * angle.cos()).ceil()).to_int_unchecked() };
		let circle_sin_x : i16 = unsafe { (300.0 + (100.0 * angle.sin()).ceil()).to_int_unchecked() };


		for event in event_pump.poll_iter() {
			match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { 
                	break 'repeat 
                },
                _ => {}
            }
		}

		println!("cos: {} sin: {} angle: {}", circle_cos_y, circle_sin_x, angle);

		display.canvas.set_draw_color(Color::RGB(20, 20, 20));
		display.canvas.clear();

		display.draw_text_centered(400, 500, &write, "ORBIT!", 16);

		let _ = display.canvas.aa_circle(400, 300, radius, circle_color);
		let _ = display.canvas.filled_circle(circle_cos_y, circle_sin_x, radius / 8, circle_color);

		angle += 0.02;

		display.canvas.present();
	}
}

