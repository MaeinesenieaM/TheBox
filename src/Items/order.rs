use sdl2::pixels::Color;
use sdl2::rect::*;

use sdl2::keyboard::*;

use thebox::*;

/*
struct Entity {
    fill: bool,
    data: i32,
    pos_x: i16,
    pos_y: i16,
}
*/

pub const NAME: &str = "Order";
pub const ID: u8 = 9;

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &Write) {
    let window = display.canvas.window();

    let points = grid_points(window.size(), 16);

    let message: Label = Label::new(
            400,
            550,
            16,
            &write,
            Some(String::from("This section will have a vector visualizer and it will order it."))
    );
    
    'repeat: loop {
        let keyboard: KeyboardState = KeyboardState::new(&sdl_context.event_pump);

        display.canvas.set_draw_color(DEFAULT_CLEAR_COLOR);
        display.canvas.clear();
        
        if keyboard.is_scancode_pressed(Scancode::Escape) {let _ = sdl_context.send_quit();}
        if sdl_context.check_quit() {break 'repeat}

        let mut count = 0;

        for point in points.iter() {
            count = count + 1;
            let pos: (i32, i32) = sdl2::rect::Point::from(*point.clone()).into();
            let rectangle = Rect::new(pos.0, pos.1, 16, 16);
            display.canvas.set_draw_color(Color::RGB(100, 120, 100));
            let _ = display.canvas.draw_rect(rectangle);
            display.canvas.set_draw_color(Color::RGB(200, 200, 200));
            let _ = display.canvas.draw_point(*point);
        }
        
        message.draw(display).unwrap();

        display.canvas.present();
    }
}

//Calculates the grid and return the Points.
fn grid_points(size: (u32, u32), difference: i32) -> Vec<Point> {
    let width: i32 = i32::try_from(size.0).unwrap();
    let height: i32 = i32::try_from(size.1).unwrap();

    let amount_x: i32 = width / difference;
    let amount_y: i32 = height / difference;

    let mut points: Vec<Point> = Vec::new();

    for i in 0..amount_y {
        for j in 0..amount_x {
            points.push(Point::new(
                (width / amount_x) * j,
                (height / amount_y) * i,
            ));
        }
    }

    points
}
