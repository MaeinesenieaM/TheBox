use sdl2::event::Event;
use sdl2::keyboard::*;
use sdl2::mouse::*;
use sdl2::pixels::Color;

use thebox::*;

pub const NAME: &str = "Input";
pub const ID: u8 = 1;

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &mut Write) {
    let window_ref = display.canvas.window();
    let (window_width, window_height): (u32, u32) = window_ref.size();

    let mut sliders: Vec<Slider<u8>> = Vec::new();
    let mut buttons: Vec<Button> = Vec::new();

    sliders.push(Slider::new(
        0,
        255,
        (window_width / 2) as i32 - 60,
        (window_height / 2) as i32,
        120,
        SliderType::SliderHorizontal,
    ));

    sliders.push(Slider::new(
        0,
        100,
        40,
        60,
        window_height - 100,
        SliderType::SliderVertical,
    ));

    sliders.push(Slider::new(
        0,
        50,
        40,
        40,
        window_width - 100,
        SliderType::SliderHorizontal,
    ));

    buttons.push(Button::new(
        false,
        (window_width - 64) as i32,
        (window_height - 64) as i32
    ));

    let mut slider_id: usize = 0;



    let mut blue: u8 = 65;
    let mut add: bool = true;

    let mut last_mouse_state: bool = false;

    'repeat: loop {
        if blue < 65 {
            add = true
        } else if blue > 185 {
            add = false
        };
        if add == true {
            blue = blue + 1
        } else {
            blue = blue - 1
        };

        display.canvas.set_draw_color(DEFAULT_CLEAR_COLOR);
        display.canvas.clear();

        let mouse: MouseState = MouseState::new(&sdl_context.event_pump);

        for event in sdl_context.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'repeat,
                _ => {}
            }
        }

        //For the user be able to change the sliders.
        if mouse.left() && slider_id == 0 {
            for slider in sliders.iter_mut().enumerate() {
                if slider.1.bar_rect().contains_point((mouse.x(), mouse.y())) {
                    slider_id = slider.0 + 1;
                }
            }
        } else if mouse.left() == false {
            slider_id = 0;
        }

        //For the user be able to click the buttons
        if mouse.left() && last_mouse_state == false {
            for button in buttons.iter_mut() {
                if button.rect().contains_point((mouse.x(), mouse.y())) {
                    button.toggle();
                }
            }
        }

        if slider_id != 0 {
            sliders
                .iter_mut()
                .nth(slider_id - 1)
                .expect("Something went wrong on reading the Slider Iter.")
                .update_from_pos((mouse.x(), mouse.y()));
        }

        //This draws the sliders.
        for slider in sliders.iter() {
            display.draw_text_centered(
                &write,
                slider.x - 20,
                slider.y - 8,
                &slider.get_value_ref().to_string(),
                8,
            );
            if slider.bar_rect().contains_point((mouse.x(), mouse.y())) {
                let _ = display.draw_outline(&slider.pivot_rect());
                let _ = display.draw_outline(&slider.bar_rect());
            }
            let _ = display.draw_slider_cl(slider, Color::RGB(30, 30, blue.clone()));
        }

        for button in buttons.iter() {
            if button.rect().contains_point((mouse.x(), mouse.y())) {
                let _ = display.draw_outline(&button.rect());
            }
            let _ = display.draw_button(button);
        }

        display.draw_text(&write, 0, 0, &mouse.x().to_string(), 8);

        display.draw_text_centered(
            &write,
            400,
            550,
            "SLIDERS! This will be an example of some user input tools.",
            8,
        );

        last_mouse_state = mouse.left();

        display.canvas.present();
    }
}
