use sdl3::keyboard::*;
use sdl3::mouse::*;
use sdl3::pixels::Color;

use thebox::*;

pub const NAME: &str = "Input";
pub const ID: u8 = 1;

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &Write) {
    let window_ref = display.canvas.window();
    let (window_width, window_height): (u32, u32) = window_ref.size();

    let mut sliders: Vec<Slider<f32>> = Vec::new();
    let mut buttons: Vec<Button> = Vec::new();

    let input_message: Label = Label::new(
        400,
        550,
        8,
        write,
        Some(String::from(
            "This is a special example showcasing the types of user inputs",
        )),
    );

    sliders.push(Slider::new(
        0.0,
        255.0,
        (window_width / 2) as i32,
        (window_height / 2) as i32,
        120,
        SliderType::SliderHorizontal,
    ));

    sliders.push(Slider::new(
        0.0,
        100.0,
        40,
        (window_height / 2) as i32,
        400,
        SliderType::SliderVertical,
    ));

    sliders.push(Slider::new(
        0.0,
        50.0,
        (window_width / 2) as i32,
        40,
        window_width - 100,
        SliderType::SliderHorizontal,
    ));

    let mut sliders_label: Label = Label::new(0, 0, 8, write, None);

    buttons.push(Button::new(
        false,
        (window_width - 64) as i32,
        (window_height - 64) as i32,
    ));

    let mut slider_id: usize = 0;

    let mut blue: u8 = 65;
    let mut add: bool = true;

    let mut last_mouse_state: bool = false;
    //    let mut last_key_pressed: String;

    'repeat: loop {
        let mouse: MouseState = MouseState::new(&sdl_context.event_pump);
        let keyboard: KeyboardState = KeyboardState::new(&sdl_context.event_pump);

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

        if keyboard.is_scancode_pressed(Scancode::Escape) {
            let _ = sdl_context.send_quit();
        }
        if sdl_context.check_quit() {
            break 'repeat;
        }

        //For the user be able to change the sliders.
        if mouse.left() && slider_id == 0 {
            for slider in sliders.iter_mut().enumerate() {
                if slider.1
                    .bar_rect()
                    .contains_point((mouse.x() as i32, mouse.y() as i32))
                {
                    slider_id = slider.0 + 1;
                }
            }
        } else if mouse.left() == false {
            slider_id = 0;
        }

        //For the user be able to click the buttons
        if mouse.left() && last_mouse_state == false {
            for button in buttons.iter_mut() {
                if button
                    .rect()
                    .contains_point((mouse.x() as i32, mouse.y() as i32))
                {
                    button.toggle();
                }
            }
        }

        if slider_id != 0 {
            let slider = sliders.iter_mut()
                .nth(slider_id - 1)
                .expect("Something went wrong on reading the Slider Iter.");
            slider.update_from_pos((mouse.x() as i32, mouse.y() as i32));
            
            println!("[{}, {}]", slider.get_cartesian(display).x, slider.get_cartesian(display).y);
        }

        //This draws the sliders.
        for slider in sliders.iter() {
            match slider.get_type() {
                SliderType::SliderHorizontal => sliders_label.set_pos(slider.x, slider.y - 20),
                SliderType::SliderVertical => sliders_label.set_pos(slider.x - 20, slider.y),
            }
            sliders_label.update_text(slider.get_value_ref().to_string());
            let _ = sliders_label.draw(display);

            if slider
                .bar_rect()
                .contains_point((mouse.x() as i32, mouse.y() as i32))
            {
                let _ = slider.draw_outline(display, COLOR_WHITE);
            }
            let _ = slider.draw_cl(display, Color::RGB(30, 30, blue.clone()));
        }

        //This draws the buttons
        for button in buttons.iter() {
            if button
                .rect()
                .contains_point((mouse.x() as i32, mouse.y() as i32))
            {
                let _ = button.draw_outline(display, COLOR_WHITE);
            }
            let _ = button.draw(display);
        }

        let _ = input_message.draw(display);

        last_mouse_state = mouse.left();

        display.canvas.present();
        display.sleep()
    }
}
