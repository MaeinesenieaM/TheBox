use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::*;

//use sdl2::gfx::primitives::DrawRenderer;

use crate::window::{Display, Write};

pub const NAME : &str = "Rays";
pub const ID : u8 = 2;

struct Arrow {
    angle: f32, //0.0 = 0, 0.25 = 45, 0.5 = 90 and so on. 
    x: i32,
    y: i32
}

struct ButtonState {
    keycode: Keycode,
    pressed: bool 
}

pub fn start(display: &mut Display, event_pump: &mut sdl2::EventPump, write: &mut Write) {


    let window_ref = display.canvas.window();
    let (window_width, window_height): (u32, u32) = window_ref.size();

    let mut arrow = Arrow {
        angle: 0.66,
        x: (window_width  / 2) as i32,
        y: (window_height / 2) as i32
    };

    let ray_lenght: f32 = 350.0;
    let ray_color: Color = Color::RGB(228, 133, 230);


    let mut left_arrow = ButtonState {
        keycode: Keycode::Right,
        pressed: false
    };
    let mut right_arrow = ButtonState {
        keycode: Keycode::Left,
        pressed: false
    };

    'repeat: loop {

        display.canvas.set_draw_color(super::super::DEFAULT_CLEAR_COLOR);
        display.canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'repeat,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => left_arrow.pressed = true,
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => right_arrow.pressed = true,
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => left_arrow.pressed = false,
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => right_arrow.pressed = false,
                _ => {}
            }
        }

        arrow.turn(&left_arrow);
        arrow.turn(&right_arrow);

        let (pos_end_x, pos_end_y): (i32, i32) = angle_pos(arrow.x, arrow.y, arrow.angle, ray_lenght);

        display.canvas.set_draw_color(ray_color);
        let _ = display.canvas.draw_line((arrow.x, arrow.y), (pos_end_x, pos_end_y));

        display.draw_text_centered(
            &write,
            400,
            550,
            "This section will be focused on making rays that bounces on the walls.",
            8,
        );
        display.canvas.present();
    }
}

fn angle_pos (x: i32, y: i32, angle: f32, distance: f32) -> (i32, i32) {
    (x + (distance * angle.sin()) as i32, y + (distance * angle.cos()) as i32)
}

impl Arrow {
    fn turn(&mut self, direction: &ButtonState) {
        if direction.pressed != true {return}
        match direction {
            ButtonState { keycode: Keycode::Right, .. } => self.angle = self.angle + 0.02,
            ButtonState { keycode: Keycode::Left, .. } => self.angle = self.angle - 0.02,
            _ => {}
        };
    }
}