pub mod corner;

pub fn select_item (choice: i32, display: &mut crate::window::Display, event_pump: &mut sdl2::EventPump) {
	match choice {
		1 => {
		corner::start(display, event_pump);
		},
		_ => {
		println! ("COULD NOT FIND ANYTHING!");
		}
	}
}