use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::*;
use sdl2::mouse::*;

use thebox::*;

pub const NAME: &str = "Input";
pub const ID: u8 = 8;

pub fn start(display: &mut Display, event_pump: &mut sdl2::EventPump, write: &mut Write) {

    let window_ref = display.canvas.window();
    let (window_width, window_height): (u32, u32) = window_ref.size();

    let mut sliders: Vec<Slider> = Vec::new();

    sliders.push(Slider::new(
        0,
        255,
        (window_width / 2) as i32 - 60,
        (window_height / 2) as i32,
        120,
        SliderType::SliderHorizontal
    ));

    sliders.push(Slider::new(
        0,
        128,
        (window_width / 2) as i32 + 120,
        (window_height / 2) as i32,
        120,
        SliderType::SliderVertical
    ));

    'repeat: loop {

        display.canvas.set_draw_color(DEFAULT_CLEAR_COLOR);
        display.canvas.clear();

        let mouse: MouseState = MouseState::new(event_pump);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'repeat,
                _ => {}
            }
        }


        for slider in sliders.iter() {
            display.draw_text_centered(
                &write,
                slider.x - 20,
                slider.y + 20,
                &slider.value.to_string(),
                8
            );
            display.canvas.set_draw_color(Color::RGB(130, 195, 60));
            let _ = display.draw_slider(slider);
        }

        if mouse.left() {
            for slider in sliders.iter_mut() {                
                if slider.bar_rect().contains_point((mouse.x(), mouse.y())) {
                    slider.update_from_pos((mouse.x(), mouse.y()));
                }
            }        
        }

        display.draw_text(
            &write,
            0,
            0,
            &mouse.x().to_string(),
            8
        );

        display.draw_text_centered(
            &write,
            400,
            550,
            "SLIDERS! This will be an example of some user input tools.",
            8,
        );

        display.canvas.present();
    }
}