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
        100,
        (window_width / 2) as i32 + 120,
        (window_height / 2) as i32,
        100,
        SliderType::SliderVertical
    ));

    sliders.push(Slider::new(
        0,
        1000,
        40,
        40,
        (window_width - 100) as u32,
        SliderType::SliderHorizontal
    ));

    let mut slider_id: usize = 0;

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

        if mouse.left() && slider_id == 0 {
            for slider in sliders.iter_mut().enumerate() {                
                if slider.1.bar_rect().contains_point((mouse.x(), mouse.y())) {
                    slider_id = slider.0 + 1;
                }
            }        
        } else if mouse.left() == false { slider_id = 0;}

        if slider_id != 0 {
            sliders.iter_mut().nth(slider_id - 1)
                .expect("Something went wrong on reading the Slider Iter.")
                .update_from_pos((mouse.x(), mouse.y()));
        }

        //This draws the sliders.
        for slider in sliders.iter() {
            display.draw_text_centered(
                &write,
                slider.x - 20,
                slider.y - 8,
                &slider.value.to_string(),
                8
            );
            display.canvas.set_draw_color(Color::RGB(130, 195, 60));
            let _ = display.draw_slider(slider);
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