use sdl2::keyboard::*;

use std::{fs, io};
use thebox::*;

pub const NAME: &str = "Textures";
pub const ID: u8 = 5;

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &Write) {
    
    let message: Label = Label::new(
            400,
            550,
            16,
            &write,
            Some(String::from("FUNNY TEXTURES!"))
    );

    let texture_creator = display.canvas.texture_creator();
    
    let png_file: Result<fs::File, io::Error> = get_asset_file("texture_test.png");

    if let Err(_) = png_file {
        let err = Label::new(120, 40, 8, write, Some(String::from("UNABLE TO READ FILE!")));
        display.canvas.clear();
        let _ = err.draw_cl(display, COLOR_RED);
        display.canvas.present();
        std::thread::sleep(std::time::Duration::from_secs(2));
        return;
    }
    
    let image = texture_from_file(png_file.unwrap(), &texture_creator).unwrap();

    let rect = sdl2::rect::Rect::new(
        display.width_center() as i32 - image.query().width as i32 / 2, 
        display.height_center() as i32 - image.query().height as i32 / 2,
        image.query().width,
        image.query().height
    );
    
    'repeat: loop {
        display.canvas.clear();

        let _ = display.canvas.copy(&image, None, rect);

        let keyboard: KeyboardState = KeyboardState::new(&sdl_context.event_pump);
        
        if keyboard.is_scancode_pressed(Scancode::Escape) {let _ = sdl_context.send_quit();}
        if sdl_context.check_quit() {break 'repeat}

        let _ = message.draw(display);

        display.canvas.present();
    }
}