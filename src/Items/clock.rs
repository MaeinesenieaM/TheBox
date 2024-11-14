use sdl2::event::Event;
use sdl2::keyboard::*;
use sdl2::pixels::Color;
use sdl2::rect::*;

use std::time;

use thebox::{Display, Write};

pub const NAME: &str = "Clock";
pub const ID: u8 = 7;

struct Clock {
    pos_x: i32,
    pos_y: i32,
    second_hand_ang: f32,
    minute_hand_ang: f32,
    hour_hand_ang: f32,
    size: f32,
    local_time: time::Duration,
}

pub fn start(display: &mut Display, event_pump: &mut sdl2::EventPump, write: &mut Write) {
    let (window_x, window_y): (u32, u32) = display.canvas.window().size();

    let window_x: i16 = window_x.try_into().unwrap();
    let window_y: i16 = window_y.try_into().unwrap();

    let screen_center: Point = Point::new(window_x as i32 / 2, window_y as i32 / 2);
//    let temp_geo: Vec<Point> = thebox::geometry(screen_center, 4, 64.0);

    'repeat: loop {
        display.canvas.set_draw_color(Color::RGB(20, 20, 20));
        display.canvas.clear();
        display.canvas.set_draw_color(thebox::COLOR_WHITE);

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

        let _ = display.draw_geometry(screen_center, 8, 64.0);

//        let _ = display
//            .canvas
//            .draw_points(temp_geo.as_slice());

        display.draw_text_centered(
            &write,
            400,
            550,
            "In here, there will be a analog clock that tells time live.",
            8,
        );

        display.canvas.present();
    }
}

impl Clock {

    /*fn new(pos_x: i32, pos_y: i32, size: f32) -> Clock {
        Clock {
            pos_x,
            pos_y,
            second_hand_ang: 0.0,
            minute_hand_ang: 0.0,
            hour_hand_ang: 0.0,
            size,
            local_time: time::Duration
        }
    }
    //This might get an error, be sure to handle it well.
    fn draw(&self, display: &mut Display) -> Result<(), String> {
        display.canvas.circle(self.pos_x, self.pos_y, self.size, thebox::COLOR_WHITE)

    }*/

    //Updates de clock hands acording to its local time.
    fn update_hands(&mut self) {
        let secs: u64 = self.local_time.as_secs();

        self.second_hand_ang = (secs % 60) as f32 / 60.0;
        self.minute_hand_ang = (secs % 3600) as f32 / 3600.0;
        self.hour_hand_ang = (secs % 43200) as f32 / 43200.0;
    }
}