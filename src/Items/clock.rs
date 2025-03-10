use sdl2::keyboard::*;
use sdl2::pixels::Color;
use sdl2::rect::*;

use std::time;

use thebox::{Display, Write, SdlContext};

pub const NAME: &str = "Clock";
pub const ID: u8 = 7;

const CLOCK_SIZE: f32 = 180.0;

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, _write: &Write) {
    let (window_x, window_y): (u32, u32) = display.canvas.window().size();

    let window_x: i16 = window_x.try_into().unwrap();
    let window_y: i16 = window_y.try_into().unwrap();

    let screen_center: Point = Point::new(window_x as i32 / 2, window_y as i32 / 2);

    let mut clock: Clock = Clock::new(screen_center, CLOCK_SIZE);

    'repeat: loop {
        display.canvas.set_draw_color(Color::RGB(20, 20, 20));
        display.canvas.clear();
        display.canvas.set_draw_color(thebox::COLOR_WHITE);
        
        let keyboard: KeyboardState = KeyboardState::new(&sdl_context.event_pump);

        if keyboard.is_scancode_pressed(Scancode::Escape) {let _ = sdl_context.send_quit();}
        if sdl_context.check_quit() {break 'repeat}
        
        clock.update_hands_real();
        let _ = clock.draw(display);

        display.canvas.present();
    }
}
#[allow(dead_code)]
struct Clock {
    pos: Point,
    second_hand_ang: f32,
    minute_hand_ang: f32,
    hour_hand_ang: f32,
    size: f32,
    local_time: time::SystemTime,
}
#[allow(dead_code)]
impl Clock {
    fn new<P: Into<Point>>(pos: P, size: f32) -> Clock {
        Clock {
            pos: pos.into(),
            second_hand_ang: 0.0,
            minute_hand_ang: 0.0,
            hour_hand_ang: 0.0,
            size,
            local_time: time::SystemTime::now()
        }
    }
    //This might get an error, be sure to handle it well.
    fn draw(&self, display: &mut Display) -> Result<(), String> {
        display.draw_geometry(self.pos, 16, self.size)?;
        display.draw_geometry_points(self.pos, 12, self.size * 0.9)?;
        display.canvas.set_draw_color(Color::RGB(180, 40, 40));
        display.draw_angle(self.pos, self.second_hand_ang, self.size * 0.8)?;
        display.canvas.set_draw_color(Color::RGB(40, 40, 180));
        display.draw_angle(self.pos, self.minute_hand_ang, self.size * 0.7)?;
        display.canvas.set_draw_color(thebox::COLOR_WHITE);
        display.draw_angle(self.pos, self.hour_hand_ang, self.size * 0.5)?;
        Ok(())
    }
    //Updates the clock hands according to its local time.
    fn update_hands(&mut self) {
        let secs: u64 = self.local_time.elapsed().unwrap().as_secs();
        self.second_hand_ang = (secs % 60) as f32 / 60.0 * 360.;
        self.minute_hand_ang = (secs % 3600) as f32 / 3600.0 * 360.;
        self.hour_hand_ang = (secs % 43200) as f32 / 43200.0 * 360.;
    }
    //Updates the clock by the OS SystemTime.
    fn update_hands_real(&mut self) {
        let secs: u64 = time::SystemTime::now()
            .duration_since(time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.second_hand_ang = (secs % 60) as f32 / 60.0 * 360.;
        self.minute_hand_ang = (secs % 3600) as f32 / 3600.0 * 360.;
        self.hour_hand_ang = (secs % 43200) as f32 / 43200.0 * 360.;
    }
}