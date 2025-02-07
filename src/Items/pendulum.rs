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

struct Pendulum {
    pos: FPoint,
    axle: FPoint,
    angle: f32,
    length: f32,
    mass: f32,
    velocity: Velocity
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
            velocity: Velocity {
                x: 0.0,
                y: 0.0
            }
        }
    }
    
    pub fn draw(&self, display: &mut Display) -> Result<(), String> {
        display.canvas.set_draw_color(DEFAULT_COLOR);
        display.canvas.draw_fline(self.pos, self.axle)?;
        //display.draw_angle_float(self.pos, self.angle, self.length)?;
        //display.draw_angle_float(self.pos, self.angle + 180.0, self.length)?;
        display.canvas.set_draw_color(DEFAULT_CLEAR_COLOR);
        Ok(())
    }
    
    pub fn update(&mut self, gravity: &f32) {
        self.velocity.y = self.velocity.y + gravity / self.mass;
    }
    
    pub fn simulate(&mut self, time: &Duration) {
        self.pos.x = self.pos.x + self.velocity.x * time.as_secs_f32();
        self.pos.y = self.pos.y + self.velocity.y * time.as_secs_f32();
    }
    
//    pub fn mass_center(&self) -> FPoint {
//        angle_fpoint(self.pos, self.angle, self.length / 2.0)
//    }
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
        FPoint::new(display.width_center() as f32, display.height_center() as f32),
        FPoint::new(display.width_center() as f32, display.height_center() as f32 - 100.0),
        0.0,
        50.0,
        5.0,
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
            pendulum.update(&gravity);
            pendulum.simulate(&time.elapsed());
            time = Instant::now();
        }
        
        for slider in sliders.iter_mut().enumerate() {
            let spc_ref = slider.1; //spc = slider_pixel_color
            let pos = slider.0;
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
            display.height_center() as i32,
            8,
            write, 
            Some(format!("{:?} | {:?}", pendulum.pos, pendulum.velocity.y))
        ).draw(display).unwrap();
        
        pendulum.draw(display).unwrap();
        let _ = message.draw(display);
        
        display.canvas.present();
    }
}