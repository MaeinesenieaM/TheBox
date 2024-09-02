pub mod orbit;
pub mod order;
pub mod rays;
pub mod textures;
pub mod audio;
pub mod clock;
pub mod tree;
pub mod input;

//This code is startig to hit a nerve on me, but it's going to be here for the long run until
//I figure something better...

use thebox::{Display, Write};

pub fn select_item(
    choice: i32,
    display: &mut Display,
    event_pump: &mut sdl2::EventPump,
    write: &mut Write,
) {
    match choice {
        0 => orbit::start(display, event_pump, write),
        1 => input::start(display, event_pump, write),
        2 => rays::start(display, event_pump, write),
        3 => tree::start(display, event_pump, write),
        4 => textures::start(display, event_pump, write),
        5 => audio::start(display, event_pump, write),
        6 => clock::start(display, event_pump, write),
        9 => order::start(display, event_pump, write),
        _ => {
            println!("COULD NOT FIND ANYTHING!");
        }
    }
}

pub fn name_item (choice: i32) -> &'static str {
	match choice {
		0 => orbit::NAME,
        1 => input::NAME,
        2 => rays::NAME,
        3 => tree::NAME,
        4 => textures::NAME,
        5 => audio::NAME,
        6 => clock::NAME,
        9 => order::NAME,
        _ => {
            "<NOTHING>"
        }
	}
}