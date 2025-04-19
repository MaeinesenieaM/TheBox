use sdl3::mouse::*;
use sdl3::keyboard::*;
use sdl3::rect::*;
use sdl3::pixels::Color;
use sdl3::render::FPoint;

use std::time::*;
use fastrand;
use thebox::*;

pub const NAME: &str = "Double Pendulum";
pub const ID: u8 = 3;

#[derive(Debug)]
struct Pendulum {
    angle: f32,
    length: f32,
    mass: f32,
    velocity: f32,
    acceleration: f32,
    axle: FPoint,
    end: FPoint
}

impl Pendulum {
    pub fn new(
        axle: FPoint,
        angle: f32,
        length: f32,
        mass: f32,
    ) -> Pendulum {
        Pendulum {
            angle: angle.to_radians(),
            length,
            mass,
            velocity: 0.0,
            acceleration: 0.0,
            axle,
            end: FPoint::new(0.0, 0.0),
        }
    }
    
    pub fn draw(&self, display: &mut Display) -> Result<(), sdl3::Error> {
        display.canvas.set_draw_color(DEFAULT_COLOR);

    //    let end_fpoint: FPoint = angle_fpoint(*self.axle, self.angle, self.length);
        let end_point: Point = Point::new(self.end.x as i32, self.end.y as i32); //THE HORROR OF AS!

        display.canvas.draw_line(self.axle, self.end)?; //Something is going to break here, I feel it...
        display.canvas.set_draw_color(COLOR_RED);
        display.draw_geometry(end_point, 16, self.mass)?;
        display.canvas.set_draw_color(DEFAULT_CLEAR_COLOR);
        Ok(())
    }
    
    pub fn update_pos(&mut self) {
        self.end = angle_fpoint(self.axle, self.angle.to_degrees(), self.length);
    }
}

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &Write) {
    
    let mut time = Instant::now();
    let main_axle: FPoint = FPoint::new(display.width_center() as f32, display.height_center() as f32 - 100.0);
    
    let mut pendulum1: Pendulum = Pendulum::new(
        main_axle,
        90.0,
        175.0,
        20.0,
    );
    pendulum1.update_pos();
    
    let mut pendulum2: Pendulum = Pendulum::new(
        pendulum1.end,
        fastrand::f32() * 90.0,
        175.0,
        20.0,
    );
    pendulum2.update_pos();
    
    let mut sliders: Vec<Slider<f32>> = Vec::new();
    sliders.push(
        Slider::new(
            -2000.0,
            2000.0,
            display.width_center() as i32 / 3 + 10,
            display.height_center() as i32 / 8,
            200,
            SliderType::SliderHorizontal
        )
    );

    sliders[0].set_value(980.7);

    let mut mouse_slider_own: Option<usize> = None;

    let tracing_length = 400;
    
    let mut tracers: Vec<FPoint> = Vec::with_capacity(tracing_length);
    for _ in 0..tracers.capacity() {
        tracers.push(FPoint::from(pendulum2.end));
    }

    'repeat: loop {
        display.canvas.clear();

        let keyboard: KeyboardState = KeyboardState::new(&sdl_context.event_pump);
        let mouse: MouseState = MouseState::new(&sdl_context.event_pump);
        
        if keyboard.is_scancode_pressed(Scancode::Escape) {let _ = sdl_context.send_quit();}
        if sdl_context.check_quit() {break 'repeat}

        //Draws and uses the sliders.
        for slider in sliders.iter_mut().enumerate() {
            let spc_ref = slider.1; //spc = slider_pixel_color
            let pos = slider.0;
            //I really need to make all this into a function.
            if spc_ref.bar_rect().contains_point((mouse.x() as i32, mouse.y() as i32)) &&
                mouse_slider_own.is_none()
            {
                if mouse.left() {mouse_slider_own = Some(pos)}
                spc_ref.draw_outline(display, COLOR_WHITE).unwrap();
            }
            if mouse_slider_own == Some(pos) {
                spc_ref.update_from_pos((mouse.x() as i32, mouse.y() as i32));
                spc_ref.draw_outline(display, COLOR_WHITE).unwrap();
            }
            spc_ref.draw_cl(display, COLOR_GRAY).unwrap();
            
            Label::new(
                spc_ref.x, 
                spc_ref.y - 22,
                8,
                write,
                Some(format!("Gravity in cm/s: {}", spc_ref.get_value_ref())),
            ).draw_cl(display, COLOR_WHITE).unwrap()
        }
        if !mouse.left() {mouse_slider_own = None}

        let gravity = -sliders[0].from_value();

        //Simulates the double pendulum every 5 milliseconds.
        if time.elapsed() >= Duration::from_millis(5) {
            pendulum1.velocity *= 0.9999; //Friction
            pendulum1.velocity *= 0.9999;

            //Based on https://www.myphysicslab.com/pendulum/double-pendulum-en.html
            let par1 = -gravity * (2f32 * pendulum1.mass + pendulum2.mass) * pendulum1.angle.sin();
            let par2 = pendulum2.mass * gravity * (pendulum1.angle - 2f32 * pendulum2.angle).sin();
            let par3 = 2f32 * (pendulum1.angle - pendulum2.angle).sin() * pendulum2.mass;
            let par4 = pendulum2.velocity.powf(2f32) * pendulum2.length + pendulum1.velocity.powf(2f32)
                * pendulum1.length * (pendulum1.angle - pendulum2.angle).cos();
            let par5 = pendulum1.length * (2f32 * pendulum1.mass + pendulum2.mass - pendulum2.mass
                * (2f32 * pendulum1.angle - 2f32 * pendulum2.angle).cos());

            pendulum1.acceleration = (par1 - par2 - par3 * par4) / par5;

            let par1 = 2f32 * (pendulum1.angle - pendulum2.angle).sin();
            let par2 = pendulum1.velocity.powf(2f32) * pendulum1.length * (pendulum1.mass + pendulum2.mass);
            let par3 = gravity * (pendulum1.mass + pendulum2.mass) * pendulum1.angle.cos();
            let par4 = pendulum2.velocity.powf(2f32) * pendulum2.length * pendulum2.mass
                * (pendulum1.angle - pendulum2.angle).cos();
            let par5 = pendulum2.length * (2f32 * pendulum1.mass + pendulum2.mass - pendulum2.mass
                * (2f32 * pendulum1.angle - 2f32 * pendulum2.angle).cos());

            pendulum2.acceleration = (par1 * (par2 + par3 + par4)) / par5;

            let delta_time = time.elapsed().as_secs_f32();

            pendulum1.velocity += pendulum1.acceleration * delta_time;
            pendulum2.velocity += pendulum2.acceleration * delta_time;

            pendulum1.angle += pendulum1.velocity * delta_time;
            pendulum2.angle += pendulum2.velocity * delta_time;

            pendulum1.update_pos();
            pendulum2.axle = pendulum1.end;
            pendulum2.update_pos();

            time = Instant::now();
        }

        tracers[0] = pendulum2.end;
        
        pendulum1.draw(display).unwrap();
        pendulum2.draw(display).unwrap();
        
        display.canvas.set_draw_color(Color::RGB(132, 112, 89));
        display.canvas.draw_lines(tracers.as_slice()).unwrap();
        tracers.rotate_right(1);
        
        display.canvas.set_draw_color(DEFAULT_CLEAR_COLOR);
        
        display.canvas.present();
        display.sleep()
    }
    display.canvas.clear();
}