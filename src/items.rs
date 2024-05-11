pub mod orbit;
use crate::window::{Display, Write};

pub fn select_item (choice: i32, display: &mut Display, event_pump: &mut sdl2::EventPump, write: &mut Write) {
	match choice {
		1 => {
		orbit::start(display, event_pump, write);
		},
		_ => {
		println! ("COULD NOT FIND ANYTHING!");
		}
	}
}