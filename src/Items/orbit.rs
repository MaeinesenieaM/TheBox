use sdl2::event::Event;
use sdl2::keyboard::*;
use sdl2::pixels::Color;
use sdl2::video::Window;
use thebox::{Display, Write, SdlContext};

pub const NAME: &str = "Orbit";
pub const ID: u8 = 0;

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &mut Write) {
    let radius: f32 = 100.0;

    let mut blue: u8 = 120;
    let mut over: bool = false;

    let mut angle: f32 = 0.0;

    let window_ref: &Window = display.canvas.window();

    let (window_x, window_y) = window_ref.size();
    let (window_x_middle, window_y_middle): (i32, i32) = (window_x as i32 / 2, window_y as i32 / 2);

    let circle_color: Color = Color::RGB(230, 80, 60);

    'repeat: loop {

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

        display.canvas.set_draw_color(Color::RGB(20, 20, 20));
        display.canvas.clear();

        write.set_draw_color_rgb(120, 120, blue);
        display.draw_text_centered(
            &write,
            window_x_middle as i32,
            (window_y_middle + 100) as i32,
            "ORBIT!",
            16,
        );

        display.canvas.set_draw_color(thebox::COLOR_WHITE);

        let _ = display.draw_geometry(
            (window_x_middle, window_y_middle),
            24,
            radius
        );

        display.canvas.set_draw_color(circle_color);

        let _ = display.draw_geometry(
            thebox::angle_point((window_x_middle, window_y_middle), angle, radius),
            16,
            16.0
        );

        if blue == 255 || blue == 0 {
            over = !over
        };

        if over == true {
            blue += 1
        } else {
            blue -= 1
        };

        angle += 0.005;

        display.draw_text(&write, 0, 0, &angle.to_string(), 8);

        display.canvas.present();
    }

    write.set_draw_color(thebox::DEFAULT_COLOR);
}
