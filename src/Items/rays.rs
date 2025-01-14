use sdl2::event::Event;
use sdl2::keyboard::*;
use sdl2::mouse::MouseState;
use sdl2::pixels::Color;

use std::f32::consts::PI;
use thebox::*;

pub const NAME: &str = "Rays";
pub const ID: u8 = 2;

struct Arrow {
    angle: f32, //0.0 = 0, 0.25 = 45, 0.5 = 90 and so on.
    x: i32,
    y: i32,
}

struct ButtonState {
    keycode: Keycode,
    pressed: bool,
}

struct Modifier {
    name: String,
    slider: Slider,
}

//I Should've use Points instead of a bunch of tuples, but I'm too lazy to rewrite it.
pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &mut Write) {
    let window_ref = display.canvas.window();
    let (window_width, window_height): (u32, u32) = window_ref.size();

    let mut arrow = Arrow {
        angle: 0.0,
        x: (window_width / 2) as i32,
        y: (window_height / 2) as i32,
    };

    let ray_color: Color = Color::RGB(228, 133, 230);

    //This single value divides how many steps it will take to calculate the collisions.
    let mut ray_precision: u32 = 0;

    //The length of each ray.
    let mut ray_length: f32 = 0.0;

    //Basically how many rays will be used. It does not mean how many corners it will hit,
    //only the amount of rays.
    let mut ray_cycles: i32 = 0;

    let mut modifiers: Vec<Modifier> = Vec::new();

    modifiers.push(Modifier {
        name: String::from("Precision"),
        slider: Slider::new(
            0,                            //Minimum
            200,                          //Max
            60,                           //X
            (window_height - 40) as i32,  //Y
            120,                          //Length
            SliderType::SliderHorizontal, //Type
        ),
    });
    modifiers.push(Modifier {
        name: String::from("Rays Lengths"),
        slider: Slider::new(
            10,
            1000,
            60,
            (window_height - 80) as i32,
            120,
            SliderType::SliderHorizontal,
        ),
    });
    modifiers.push(Modifier {
        name: String::from("Rays Quantity"),
        slider: Slider::new(
            0,
            100,
            60,
            (window_height - 120) as i32,
            120,
            SliderType::SliderHorizontal,
        ),
    });

    let mut set_iter = modifiers.iter_mut();
    set_iter.next().unwrap().slider.set_value(100);
    set_iter.next().unwrap().slider.set_value(400);
    set_iter.next().unwrap().slider.set_value(6);

    let mut left_arrow = ButtonState {
        keycode: Keycode::Right,
        pressed: false,
    };
    let mut right_arrow = ButtonState {
        keycode: Keycode::Left,
        pressed: false,
    };

    let mut slider_id: usize = 0;

    'repeat: loop {
        //Responsible for updating values.
        let mut set_iter = modifiers.iter();
        set_iter
            .next()
            .unwrap()
            .slider
            .mut_from_value(&mut ray_precision);
        set_iter
            .next()
            .unwrap()
            .slider
            .mut_from_value(&mut ray_length);
        set_iter
            .next()
            .unwrap()
            .slider
            .mut_from_value(&mut ray_cycles);

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

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => left_arrow.pressed = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => right_arrow.pressed = true,
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => left_arrow.pressed = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => right_arrow.pressed = false,
                _ => {}
            }
        }

        //Check input of mouse in the slider.
        //TODO! Make all this into a simpler function.
        if mouse.left() && slider_id == 0 {
            for modifier in modifiers.iter_mut().enumerate() {
                if modifier
                    .1
                    .slider
                    .bar_rect()
                    .contains_point((mouse.x(), mouse.y()))
                {
                    slider_id = modifier.0 + 1;
                }
            }
        } else if mouse.left() == false {
            slider_id = 0;
        }

        if slider_id != 0 {
            modifiers
                .iter_mut()
                .nth(slider_id - 1)
                .expect("Something went wrong on reading the Slider Iter.")
                .slider
                .update_from_pos((mouse.x(), mouse.y()));
        }

        //self-explanatory
        arrow.turn(&left_arrow);
        arrow.turn(&right_arrow);
        let mut next_angle: f32 = arrow.angle;

        let (mut pos_end_x, mut pos_end_y): (i32, i32) = ray_corner(
            (arrow.x, arrow.y),
            (window_width, window_height),
            &mut next_angle,
            ray_precision,
            ray_length,
        );

        //Draws the main ray.
        display.canvas.set_draw_color(ray_color);
        let _ = display
            .canvas
            .draw_line((arrow.x, arrow.y), (pos_end_x, pos_end_y));

        display.canvas.set_draw_color(Color::RGB(26, 90, 186));

        for _ in 0..ray_cycles {
            let (temp_x, temp_y): (i32, i32) = ray_corner(
                (pos_end_x, pos_end_y),
                (window_width, window_height),
                &mut next_angle,
                ray_precision,
                ray_length,
            );
            let _ = display
                .canvas
                .draw_line((pos_end_x, pos_end_y), (temp_x, temp_y));
            pos_end_x = temp_x;
            pos_end_y = temp_y;
        }

        display.draw_text(&write, 0, 0, &format!("Angle: {}", arrow.angle), 8);

        for modifier in modifiers.iter() {
            display.draw_text_centered(
                &write,
                modifier.slider.x - 20,
                modifier.slider.y - 8,
                &modifier.slider.get_value_ref().to_string(),
                8,
            );
            display.draw_text(
                &write,
                modifier.slider.x,
                modifier.slider.y - 22,
                &modifier.name,
                8,
            );

            let _ = display.draw_slider_cl(&modifier.slider, Color::RGB(30, 110, 40));
        }

        display.canvas.present();
    }
}

//return a position based on the angle.
fn angle_pos(x: i32, y: i32, mut angle: f32, distance: f32) -> (i32, i32) {
    angle = PI * angle;
    (
        x + (distance * angle.sin()) as i32,
        y + (distance * angle.cos()) as i32,
    )
}

fn rounder(value: &mut i32, max: u32) {
    if *value > max as i32 {
        *value = max as i32
    } else if *value < 0i32 {
        *value = 0i32
    }
}

//This function simulates a ray and returns the possible collision position.
//Along with changing the angle inserted.
fn ray_corner(
    (x, y): (i32, i32),
    (width, height): (u32, u32),
    angle: &mut f32,
    precision: u32,
    ray_length: f32,
) -> (i32, i32) {
    for i in 1..(precision + 1) {
        let (mut px, mut py) = angle_pos(x, y, *angle, (ray_length / precision as f32) * i as f32);
        if px > width as i32 || px < 0 {
            rounder(&mut px, width);
            *angle = *angle * -1.0;
            return (px, py);
        } else if py > height as i32 || py < 0 {
            rounder(&mut py, height);
            *angle = *angle * -1.0 + 1.0;
            return (px, py);
        }
    }

    angle_pos(x, y, *angle, ray_length)
}

impl Arrow {
    fn turn(&mut self, direction: &ButtonState) {
        if direction.pressed != true {
            return;
        }
        match direction {
            ButtonState {
                keycode: Keycode::Right,
                ..
            } => self.angle = self.angle + 0.0020,
            ButtonState {
                keycode: Keycode::Left,
                ..
            } => self.angle = self.angle - 0.0020,
            _ => {}
        };
    }
}
