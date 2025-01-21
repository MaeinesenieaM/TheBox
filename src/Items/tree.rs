//use sdl2::pixels::Color;
use sdl2::keyboard::*;
use thebox::*;

pub const NAME: &str = "Tree";
pub const ID: u8 = 4;

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &Write) {
    
    let message: Label = Label::new(
            400,
            550,
            16,
            &write,
            Some(String::from("There will be procedural a tree in here."))
    );
    
    'repeat: loop {
        display.canvas.clear();
        
        let keyboard: KeyboardState = KeyboardState::new(&sdl_context.event_pump);
        
        if keyboard.is_scancode_pressed(Scancode::Escape) {let _ = sdl_context.send_quit();}
        if sdl_context.check_quit() {break 'repeat}

        let _ = message.draw(display);

        display.canvas.present();
    }
}
