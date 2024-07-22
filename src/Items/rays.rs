use sdl2::event::Event;
use sdl2::pixels::Color;
//use sdl2::rect::Point;
use sdl2::keyboard::*;

//use sdl2::gfx::primitives::DrawRenderer;

use crate::window::{Display, Write};
use std::f32::consts::PI;

pub const NAME : &str = "Rays";
pub const ID : u8 = 2;

const RAY_LENGHT : f32 = 600.0;
const RAY_PRECISION : u32 = 150;
const RAY_CYCLES : i32 = 32;

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
        angle: 0.0,
        x: (window_width  / 2) as i32,
        y: (window_height / 2) as i32
    };

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

        //self explanatory
        arrow.turn(&left_arrow);
        arrow.turn(&right_arrow);

        let mut next_angle: f32 = arrow.angle;

        let (mut pos_end_x, mut pos_end_y): (i32, i32) = ray_corner(
            (arrow.x, arrow.y),
            (window_width, window_height),
            &mut next_angle, 
            RAY_PRECISION
        );

        display.canvas.set_draw_color(ray_color);
        let _ = display.canvas.draw_line((arrow.x, arrow.y), (pos_end_x, pos_end_y));

        display.canvas.set_draw_color(Color::RGB(26, 90, 186));

        for _ in 0..RAY_CYCLES {
            let (temp_x, temp_y): (i32, i32) = ray_corner(
                (pos_end_x, pos_end_y),
                (window_width, window_height),
                &mut next_angle,
                RAY_PRECISION,
            );
            let _ = display.canvas.draw_line((pos_end_x, pos_end_y), (temp_x, temp_y));
            pos_end_x = temp_x;
            pos_end_y = temp_y;
        };

        display.draw_text_centered(
            &write,
            400,
            550,
            "This section will be focused on making rays that bounces on the walls.",
            8,
        );

        display.draw_text(
            &write,
            0,
            0,
            &format!("Angle: {}", arrow.angle),
            8,
        );
        display.canvas.present();
    }
}

//return a position based on the angle.
fn angle_pos(x: i32, y: i32, mut angle: f32, distance: f32) -> (i32, i32) {
    angle = PI * angle;
    (x + (distance * angle.sin()) as i32, y + (distance * angle.cos()) as i32)
}

fn rounder(value: &mut i32, max: u32) {
    if *value > max as i32 { *value = max as i32 }
    else if *value < 0 as i32 { *value = 0 as i32 }
}

//This function simulates a ray and returns the possible collision position.
//Along with changing the angle inserted.
fn ray_corner(
    (x, y): (i32, i32), 
    (width, height): (u32, u32), 
    angle: &mut f32,
    precision: u32,
) -> (i32, i32) {
    for i in 1..(precision + 1) {

        let (mut px, mut py) = angle_pos(x, y, *angle, (RAY_LENGHT / precision as f32) * i as f32);
        if px > width as i32 || px < 0 {
            rounder(&mut px, width);
            *angle = *angle * -1.0;
            return (px, py)
        }
        else if py > height as i32 || py < 0 {
            rounder(&mut py, height);
            *angle = *angle * -1.0 + 1.0;
            return (px, py)
        }
    }

    angle_pos(x, y, *angle, RAY_LENGHT)
}

impl Arrow {
    fn turn(&mut self, direction: &ButtonState) {
        if direction.pressed != true {return}
        match direction {
            ButtonState { keycode: Keycode::Right, .. } => self.angle = self.angle + 0.0015,
            ButtonState { keycode: Keycode::Left, .. } => self.angle = self.angle - 0.0015,
            _ => {}
        };
    }
}