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

use thebox::{Display, Write, SdlContext};

pub fn select_item(
    choice: i32,
    display: &mut Display,
    sdl_context: &mut SdlContext,
    write: &mut Write,
) {
    match choice {
        0 => orbit::start(display, sdl_context, write),
        1 => input::start(display, sdl_context, write),
        2 => rays::start(display, sdl_context, write),
        3 => pendulum::start(display, sdl_context, write),
        4 => tree::start(display, sdl_context, write),
        5 => textures::start(display, sdl_context, write),
        6 => audio::start(display, sdl_context, write),
        7 => clock::start(display, sdl_context, write),
        9 => order::start(display, sdl_context, write),
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
