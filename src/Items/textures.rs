use sdl3::keyboard::*;
use sdl3::pixels::Color;

use std::{fs, io};
use sdl3::mouse::MouseState;
use thebox::*;

pub const NAME: &str = "Textures";
pub const ID: u8 = 5;

struct SliderPixelColor {
    slider: Slider<u8>,
    name: String,
    color: Color
}

impl SliderPixelColor {
    fn new<S: Into<String>>(
        x: i32,
        y: i32,
        length: u32,
        slider_type: SliderType,
        name: S,
        color: Color
    ) -> SliderPixelColor {
        let slider = Slider::new(0, 255, x, y, length, slider_type);
        SliderPixelColor {
            slider,
            name: name.into(),
            color
        }
    }
}

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &Write) {
    
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
    
    let mut image = texture_from_file(png_file.unwrap(), &texture_creator).unwrap();

    let rect = sdl3::rect::Rect::new(
        display.width_center() as i32 - image.query().width as i32 / 2, 
        display.height_center() as i32 - image.query().height as i32 / 2,
        image.query().width,
        image.query().height
    );

    let mut sliders: Vec<SliderPixelColor> = Vec::with_capacity(3);

    sliders.push(SliderPixelColor::new(
        display.width_center() as i32 / 3 + 10,
        display.height_center() as i32 / 8,
        256,
        SliderType::SliderHorizontal,
        "red:",
        COLOR_RED
    ));
    sliders.push(SliderPixelColor::new(
        display.width_center() as i32 / 3 + 10,
        display.height_center() as i32 / 8 + 20,
        256,
        SliderType::SliderHorizontal,
        "green:",
        COLOR_GREEN
    ));
    sliders.push(SliderPixelColor::new(
        display.width_center() as i32 / 3 + 10,
        display.height_center() as i32 / 8 + 40,
        256,
        SliderType::SliderHorizontal,
        "blue:",
        COLOR_BLUE
    ));
    sliders.push(SliderPixelColor::new(
        display.width_center() as i32 / 3 + 10,
        display.height_center() as i32 / 8 + 60,
        256,
        SliderType::SliderHorizontal,
        "alpha:",
        COLOR_GRAY
    ));
    
    for slider in sliders.iter_mut() {slider.slider.set_value(255)}
    let mut mouse_slider_own: Option<usize> = None;
    
    'repeat: loop {
        display.canvas.set_draw_color(DEFAULT_CLEAR_COLOR);
        display.canvas.clear();

        let keyboard: KeyboardState = KeyboardState::new(&sdl_context.event_pump);
        let mouse: MouseState = MouseState::new(&sdl_context.event_pump);


        if keyboard.is_scancode_pressed(Scancode::Escape) {let _ = sdl_context.send_quit();}
        if sdl_context.check_quit() {break 'repeat}

        //This makes possible to use the mouse freely on the sliders.
        for color_input in sliders.iter_mut().enumerate() {
            let spc_ref = color_input.1; //spc = slider_pixel_color
            let pos = color_input.0;
            if spc_ref.slider.bar_rect().contains_point((mouse.x() as i32, mouse.y() as i32)) &&
                mouse_slider_own.is_none()
            {
                if mouse.left() {mouse_slider_own = Some(pos)}
                spc_ref.slider.draw_outline(display, COLOR_WHITE).unwrap();
            }
            if mouse_slider_own == Some(pos) {
                spc_ref.slider.update_from_pos((mouse.x() as i32, mouse.y() as i32));
                spc_ref.slider.draw_outline(display, COLOR_WHITE).unwrap();
            }
            spc_ref.slider.draw_cl(display, spc_ref.color).unwrap();
            
            Label::new(
                spc_ref.slider.x + 180, 
                spc_ref.slider.y - 9,
                8,
                write,
                Some(format!("{} {}", spc_ref.name, spc_ref.slider.get_value_ref())),
            ).draw_cl(display, spc_ref.color).unwrap()
        }
        if !mouse.left() {mouse_slider_own = None}
        
        image.set_color_mod(
            sliders[0].slider.from_value(),
            sliders[1].slider.from_value(),
            sliders[2].slider.from_value(),
        );
        image.set_alpha_mod(sliders[3].slider.from_value());
        
        display.canvas.copy(&image, None, rect).unwrap();
        display.canvas.present();
        display.sleep()
    }
}