pub mod orbit;
pub mod order;
pub mod rays;
pub mod textures;
pub mod audio;
pub mod clock;
pub mod tree;

use thebox::{Display, Write};

pub fn select_item(
    choice: i32,
    display: &mut Display,
    event_pump: &mut sdl2::EventPump,
    write: &mut Write,
) {
    match choice {
        1 => orbit::start(display, event_pump, write),
        2 => rays::start(display, event_pump, write),
        3 => order::start(display, event_pump, write),
        4 => textures::start(display, event_pump, write),
        5 => audio::start(display, event_pump, write),
        6 => clock::start(display, event_pump, write),
        7 => tree::start(display, event_pump, write),
        _ => {
            println!("COULD NOT FIND ANYTHING!");
        }
    }
}

pub fn name_item (choice: i32) -> &'static str {
	match choice {
		1 => orbit::NAME,
        2 => rays::NAME,
        3 => order::NAME,
        4 => textures::NAME,
        5 => audio::NAME,
        6 => clock::NAME,
        7 => tree::NAME,
        _ => {
            "<NOTHING>"
        }
	}
}