use sdl2::event::Event;
use sdl2::keyboard::*;


pub fn start (display: &mut crate::window::Display, event_pump: &mut sdl2::EventPump) {
	'repeat: loop {

		for event in event_pump.poll_iter() {
			match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { 
                	break 'repeat 
                },
                _ => {}
            }
		}

		display.canvas.clear();
		display.create_text_centered(400, 400, "BOO!", 32);
		display.canvas.present();
	}
}

