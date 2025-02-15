//use sdl2::pixels::Color;
use sdl2::mouse::*;
use sdl2::keyboard::*;
use sdl2::rect::*;

use std::time::*;

use thebox::*;

pub const NAME: &str = "Double Pendulum";
pub const ID: u8 = 3;

struct Velocity {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Pendulum {
    pos: FPoint,
    axle: FPoint,
    angle: f32,
    length: f32,
    mass: f32,
    velocity: f32,
    acceleration: f32
}

impl Pendulum {
    pub fn new(
        pos: FPoint,
        axle: FPoint,
        angle: f32,
        length: f32,
        mass: f32,
    ) -> Pendulum {
        Pendulum {
            pos,
            axle,
            angle,
            length,
            mass,
            velocity: 0.0,
            acceleration: 0.0
        }
    }
    
    pub fn draw(&self, display: &mut Display) -> Result<(), String> {
        display.canvas.set_draw_color(DEFAULT_COLOR);

        let end_fpoint: FPoint = angle_fpoint(self.axle, self.angle, self.length);
        let end_point: Point = Point::new(end_fpoint.x as i32, end_fpoint.y as i32); //THE HORROR!

        display.canvas.draw_fline(self.axle, end_fpoint)?;
        display.canvas.set_draw_color(COLOR_RED);
        display.draw_geometry(end_point, 16, 16.0)?;
        display.canvas.set_draw_color(DEFAULT_CLEAR_COLOR);
        Ok(())
    }
    
    pub fn simulate(&mut self, time: &Duration, gravity: &f32) {
        self.acceleration = (gravity * self.mass) * self.angle.to_radians().sin();
        self.velocity += self.acceleration * time.as_secs_f32();
        self.angle += self.velocity * time.as_secs_f32();
    }
}

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &Write) {

    let message: Label = Label::new(
            400,
            550,
            16,
            &write,
            Some(String::from("Will simulate a double pendulum."))
    );
    
    let mut time = Instant::now();
    let mut gravity: f32 = 0.0;
    
    let mut pendulum: Pendulum = Pendulum::new(
        FPoint::new(display.width_center() as f32 + 50.0, display.height_center() as f32),
        FPoint::new(display.width_center() as f32, display.height_center() as f32 - 100.0),
        90.0,
        200.0,
        50.0,
    );
    
    let mut sliders: Vec<Slider<f32>> = Vec::new();
    sliders.push(
        Slider::new(
            -1.0,
            1.0,
            display.width_center() as i32 / 3 + 10,
            display.height_center() as i32 / 8,
            200,
            SliderType::SliderHorizontal
        )
    );
    sliders[0].set_value(0.0);
    
    let mut mouse_slider_own: Option<usize> = None;
    
    'repeat: loop {
        display.canvas.clear();

        let keyboard: KeyboardState = KeyboardState::new(&sdl_context.event_pump);
        let mouse: MouseState = MouseState::new(&sdl_context.event_pump);
        
        if keyboard.is_scancode_pressed(Scancode::Escape) {let _ = sdl_context.send_quit();}
        if sdl_context.check_quit() {break 'repeat}

        gravity = sliders[0].from_value();
        
        if time.elapsed() >= Duration::from_millis(5) {
            pendulum.simulate(&time.elapsed(), &gravity);
            time = Instant::now();
        }

        //Draws and uses the sliders.
        for slider in sliders.iter_mut().enumerate() {
            let spc_ref = slider.1; //spc = slider_pixel_color
            let pos = slider.0;
            //I really need to make all this into a function.
            if spc_ref.bar_rect().contains_point((mouse.x(), mouse.y())) &&
                mouse_slider_own.is_none()
            {
                if mouse.left() {mouse_slider_own = Some(pos)}
                spc_ref.draw_outline(display, COLOR_WHITE).unwrap();
            }
            if mouse_slider_own == Some(pos) {
                spc_ref.update_from_pos((mouse.x(), mouse.y()));
                spc_ref.draw_outline(display, COLOR_WHITE).unwrap();
            }
            spc_ref.draw_cl(display, COLOR_GRAY).unwrap();
            
            Label::new(
                spc_ref.x, 
                spc_ref.y - 22,
                8,
                write,
                Some(format!("Gravity: {}", spc_ref.get_value_ref())),
            ).draw_cl(display, COLOR_WHITE).unwrap()
        }
        if !mouse.left() {mouse_slider_own = None}
        
        Label::new(
            display.width_center() as i32,
            32,
            8,
            write, 
            Some(format!("{:?} | {:?}", pendulum.pos, pendulum.velocity))
        ).draw(display).unwrap();
        
        pendulum.draw(display).unwrap();
        let _ = message.draw(display);
        
        display.canvas.present();
    }
}