pub mod audio;
pub mod clock;
pub mod input;
pub mod orbit;
pub mod order;
pub mod rays;
pub mod textures;
pub mod tree;
pub mod pendulum;

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
        3 => pendulum::start(display, event_pump, write),
        4 => tree::start(display, event_pump, write),
        5 => textures::start(display, event_pump, write),
        6 => audio::start(display, event_pump, write),
        7 => clock::start(display, event_pump, write),
        9 => order::start(display, event_pump, write),
        _ => {
            println!("COULD NOT FIND ANYTHING!");
        }
    }
}

pub fn name_item(choice: i32) -> &'static str {
    match choice {
        0 => orbit::NAME,
        1 => input::NAME,
        2 => rays::NAME,
        3 => pendulum::NAME,
        4 => tree::NAME,
        5 => textures::NAME,
        6 => audio::NAME,
        7 => clock::NAME,
        9 => order::NAME,
        _ => "<NOTHING>",
    }
}
