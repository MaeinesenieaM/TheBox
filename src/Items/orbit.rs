use sdl2::keyboard::*;
use sdl2::pixels::Color;
use sdl2::video::Window;
use thebox::*;

pub const NAME: &str = "Orbit";
pub const ID: u8 = 0;

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &Write) {
    let radius: f32 = 100.0;

    let mut blue: u8 = 120;
    let mut over: bool = false;

    let mut angle: f32 = 0.0;

    let window_ref: &Window = display.canvas.window();

    let (window_x, window_y) = window_ref.size();
    let (window_x_middle, window_y_middle): (i32, i32) = (window_x as i32 / 2, window_y as i32 / 2);

    let circle_color: Color = Color::RGB(230, 80, 60);

    let orbit_message: Label = Label::new(
        (window_x / 2) as i32,
        window_y_middle + 100,
        16,
        &write,
        Some(String::from("ORBIT!"))
    );
    
    'repeat: loop {

        let keyboard: KeyboardState = KeyboardState::new(&sdl_context.event_pump);
        
        if keyboard.is_scancode_pressed(Scancode::Escape) {let _ = sdl_context.send_quit();}
        if sdl_context.check_quit() {break 'repeat}

        display.canvas.set_draw_color(Color::RGB(20, 20, 20));
        display.canvas.clear();
        
        let _ = orbit_message.draw_cl(display, Color::RGB(120, 120, blue));
        
        display.canvas.set_draw_color(COLOR_WHITE);

        let _ = display.draw_geometry(
            (window_x_middle, window_y_middle),
            24,
            radius
        );

        display.canvas.set_draw_color(circle_color);

        let _ = display.draw_geometry(
            angle_point((window_x_middle, window_y_middle), angle, radius),
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

        angle += 1.0;

        display.canvas.present();
    }
}
