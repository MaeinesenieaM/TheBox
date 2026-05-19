use super::*;

macro_rules! register_modules {
    ($( $id:expr => $mod_name:ident ),* $(,)?) => {
        // 1. Automatically declare the modules
        $( pub mod $mod_name; )*

        // 2. Generate the selection function
        pub fn select_item(
            choice: i32,
            display: &mut BoxDisplay,
            sdl_context: &mut SdlContext,
            write: &Write,
        ) {
            match choice {
                $( $id => $mod_name::start(display, sdl_context, write), )*
                _ => {
                    println!("COULD NOT FIND ANYTHING!");
                }
            }
        }

        // 3. Generate the naming function
        pub fn name_item(choice: i32) -> &'static str {
            match choice {
                $( $id => $mod_name::NAME, )*
                _ => "<NOTHING>",
            }
        }
    };
}

register_modules! {
    0 => orbit,
    1 => input,
    2 => rays,
    3 => pendulum,
    4 => tree,
    5 => textures,
    6 => audio,
    7 => clock,
    9 => order
}