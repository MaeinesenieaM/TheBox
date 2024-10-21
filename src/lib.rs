use sdl2::pixels::Color;
use sdl2::ttf;

use sdl2::Sdl;
use sdl2::{EventPump, VideoSubsystem};

use sdl2::gfx::framerate::FPSManager;
use sdl2::rect::*;
use sdl2::render::*;
use sdl2::video::WindowContext;

use std::f32::consts::PI;

pub const DEFAULT_COLOR: Color = Color::RGB(210, 210, 220);
pub const DEFAULT_CLEAR_COLOR: Color = Color::RGB(20, 20, 20);
pub const COLOR_WHITE: Color = Color::RGB(194, 194, 194);

const SLIDER_PIVOT_COLOR: Color = Color::RGB(120, 120, 120);
const SLIDER_PIVOT_SIZE: u32 = 14;
const SLIDER_BAR_SIZE: u32 = 8;

const BUTTON_STATE_TRUE: Color = Color::RGB(30, 165, 30);
const BUTTON_STATE_FALSE: Color = Color::RGB(165, 30, 30);
const BUTOON_DEFAULT_COLOR: Color = Color::RGB(120, 120, 120);
const BUTTON_RECT_SIZE: u32 = 24;
const BUTTON_RECT_STATE_SIZE: u32 = 16;

pub struct SdlContext {
    pub sdl2: Sdl,
    pub event_pump: EventPump,
    pub video_subsystem: VideoSubsystem,
}

pub struct Display {
    pub canvas: WindowCanvas,
    pub texture_creator: TextureCreator<WindowContext>,
    pub fps_manager: FPSManager,
}

pub struct Write<'t, 'f> {
    pub ttf: &'t ttf::Sdl2TtfContext,
    pub font: ttf::Font<'t, 'f>,
    pub color: Color,
}

pub enum SliderType {
    SliderHorizontal,
    SliderVertical,
}

//The slider is a user input element, where the user moves a pivot o control the value.
pub struct Slider {
    value: u16,
    min: u16,
    max: u16,
    pub x: i32,
    pub y: i32,
    length: u32,
    slider_type: SliderType,
}

pub struct Button {
    pub state: bool,
    pub x: i32,
    pub y: i32,
}

impl SdlContext {
    pub fn init_context() -> SdlContext {
        let sdl2 = sdl2::init().unwrap();
        let event_pump = sdl2.event_pump().unwrap();
        let video_subsystem = sdl2.video().unwrap();

        SdlContext {
            sdl2,
            event_pump,
            video_subsystem,
        }
    }
}

impl Display {
    pub fn init_display(video_subsystem: &VideoSubsystem, width: u32, height: u32) -> Display {
        let window = video_subsystem
            .window("BOX", width, height)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().present_vsync().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let fps_manager = FPSManager::new();
        Display {
            canvas,
            texture_creator,
            fps_manager,
        }
    }

    //Draws a text with the given string.
    pub fn draw_text(&mut self, write: &Write, x: i32, y: i32, string: &str, size: u32) {
        let texture = write.create_text(&self.texture_creator, string);
        let string_len: u32 = string.len().try_into().unwrap();

        let area = sdl2::rect::Rect::new(x, y, string_len * size, size * 2);
        let _ = self.canvas.copy(&texture, None, area);
    }

    //Same as above but centered.
    pub fn draw_text_centered(&mut self, write: &Write, x: i32, y: i32, string: &str, size: u32) {
        let texture = write.create_text(&self.texture_creator, string);
        let string_len: u32 = string.len().try_into().unwrap();

        let middle: i32 = ((string_len * size) / 2).try_into().unwrap(); //I hate this middle variable.

        let area = sdl2::rect::Rect::new(x - middle, y, string_len * size, size * 2);
        let _ = self.canvas.copy(&texture, None, area);
    }

    pub fn draw_outline(&mut self, rect: &Rect) -> Result<(), String> {        
        let outline: Rect = Rect::new(
            rect.x() - 2,
            rect.y() - 2,
            rect.width() + 4,
            rect.height() + 4,
        );
        self.canvas.set_draw_color(COLOR_WHITE);
        self.canvas.fill_rect(outline)?;
        Ok(())
    }

    pub fn draw_geometry<P: Into<Point>>(&mut self, pos: P, vertices: u16, size: f32) {
        let pos: Point = pos.into();
        let mut vert: Vec<Point> = geometry(pos, vertices, size);

        vert.push(vert.first().unwrap().clone());

        let _ = self
            .canvas
            .draw_lines(vert.as_slice());
    }

    //Draws a Slider on screen according to its values.
    pub fn draw_slider(&mut self, slider: &Slider) -> Result<(), String> {
        self.canvas.fill_rect(slider.bar_rect())?;
        self.canvas.set_draw_color(SLIDER_PIVOT_COLOR);
        self.canvas.fill_rect(slider.pivot_rect())?;
        self.canvas.set_draw_color(DEFAULT_COLOR);
        Ok(())
    }

    //Same as above, but is drawn with a predefined color.
    pub fn draw_slider_cl(&mut self, slider: &Slider, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(slider.bar_rect())?;
        self.canvas.set_draw_color(SLIDER_PIVOT_COLOR);
        self.canvas.fill_rect(slider.pivot_rect())?;
        self.canvas.set_draw_color(DEFAULT_COLOR);
        Ok(())
    }

    pub fn draw_button(&mut self, button: &Button) -> Result<(), String> {
        self.canvas.set_draw_color(BUTOON_DEFAULT_COLOR);
        self.canvas.fill_rect(button.rect())?;
        self.canvas.set_draw_color(button.get_state_color());
        self.canvas.fill_rect(button.state_rect())?;
        self.canvas.set_draw_color(DEFAULT_COLOR);
        Ok(())
    }
}

impl Write<'_, '_> {
    pub fn init_write<'t, 'f>(ttf: &'t ttf::Sdl2TtfContext, color: Color) -> Write<'t, 'f> {
        let font = match ttf.load_font("./src/main_assets/Fixedsys.ttf", 32) {
            Ok(font_src) => font_src,
            Err(_) => match ttf.load_font("./main_assets/Fixedsys.ttf", 32) {
                Ok(font_ass) => font_ass,
                Err(damn) => {
                    panic!("ERROR: {}", &damn)
                }
            },
        };
        let color = color;
        Write { ttf, font, color }
    }

    //This should be only used by the Display for now.
    fn create_text<'b>(
        &self,
        texture_creator: &'b TextureCreator<WindowContext>,
        string: &str,
    ) -> Texture<'b> {
        self.font
            .render(string)
            .solid(self.color)
            .unwrap()
            .as_texture(texture_creator)
            .unwrap()
    }

    pub fn set_font(&mut self, path: &str) {
        self.font = self.ttf.load_font(path, 32).expect("COULD NOT FIND FONT!");
    }

    pub fn set_draw_color_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.color = Color::RGB(r, g, b);
    }

    pub fn set_draw_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Slider {
    pub fn new(min: u16, max: u16, x: i32, y: i32, length: u32, slider_type: SliderType) -> Slider {
        Slider {
            min,
            max,
            value: min,
            x,
            y,
            length,
            slider_type,
        }
    }

    pub fn set_value(&mut self, value: u16) {
        self.value = value;
    }

    //Recommended for controled values.
    pub fn set_value_limited(&mut self, value: u16) {
        if value < self.min {
            self.value = self.min
        } else if value > self.max {
            self.value = self.max
        } else {
            self.value = value
        };
    }

    pub fn get_value_ref(&self) -> &u16 {
        &self.value
    }

    pub fn from_value<T>(&self) -> T
    where
        T: From<u16>,
    {
        T::from(self.value)
    }

    //Mutate the given value from the value of the slider.
    pub fn mut_from_value<T>(&self, value: &mut T)
    where
        T: From<u16>,
    {
        *value = T::from(self.value);
    }

    pub fn get_type(&self) -> &SliderType {
        &self.slider_type
    }

    //Returns how filled is the slider.
    pub fn percentage(&self) -> f32 {
        self.value as f32 / self.max as f32
    }

    //Calculates and returns the position of the pivot.
    pub fn pivot(&self) -> Point {
        match &self.slider_type {
            SliderType::SliderHorizontal => {
                Point::new((self.x as f32 * self.percentage()) as i32, self.y)
            }
            SliderType::SliderVertical => {
                Point::new(self.x, (self.y as f32 * self.percentage()) as i32)
            }
        }
    }

    //Returns the Rect of the pivot.
    pub fn pivot_rect(&self) -> Rect {
        let half_size: i32 = (SLIDER_PIVOT_SIZE / 2) as i32;
        match self.slider_type {
            SliderType::SliderHorizontal => Rect::new(
                self.x + (self.length as f32 * self.percentage()) as i32 - half_size,
                self.y - half_size,
                SLIDER_PIVOT_SIZE,
                SLIDER_PIVOT_SIZE,
            ),
            SliderType::SliderVertical => Rect::new(
                self.x - half_size,
                self.y + (self.length as f32 * self.percentage()) as i32 - half_size,
                SLIDER_PIVOT_SIZE,
                SLIDER_PIVOT_SIZE,
            ),
        }
    }

    //Returns the Rect of the Bar.
    pub fn bar_rect(&self) -> Rect {
        match self.slider_type {
            SliderType::SliderHorizontal => Rect::new(
                self.x,
                self.y - SLIDER_BAR_SIZE as i32 / 2,
                self.length,
                SLIDER_BAR_SIZE,
            ),
            SliderType::SliderVertical => Rect::new(
                self.x - SLIDER_BAR_SIZE as i32 / 2,
                self.y,
                SLIDER_BAR_SIZE,
                self.length,
            ),
        }
    }

    pub fn update_from_pos<P: Into<Point>>(&mut self, point: P) {
        let point = point.into();
        let distance: i32;

        match self.slider_type {
            SliderType::SliderHorizontal => distance = point.x() - self.x,
            SliderType::SliderVertical => distance = point.y() - self.y,
        }
        self.set_value_limited(
            int_from_percentage(
                &(self.max as i32),
                &percentage_from_int(&distance, &(self.length as i32)),
            )
            .try_into()
            .unwrap(),
        );
    }
}

impl Button {
    pub fn new(state: bool, x: i32, y: i32) -> Button {
        Button { state, x, y }
    }

    pub fn set_state(&mut self, state: bool) {
        self.state = state;
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn get_state_color(&self) -> Color {
        if self.state {
            BUTTON_STATE_TRUE
        } else {
            BUTTON_STATE_FALSE
        }
    }

    pub fn set_pos<P: Into<Point>>(&mut self, point: P) {
        let point = point.into();
        self.x = point.x();
        self.y = point.y();
    }

    pub fn toggle(&mut self) {
        self.state = !self.state;
    }

    pub fn rect(&self) -> Rect {
        Rect::new(
            self.x - BUTTON_RECT_SIZE as i32 / 2,
            self.y - BUTTON_RECT_SIZE as i32 / 2,
            BUTTON_RECT_SIZE,
            BUTTON_RECT_SIZE,
        )
    }

    pub fn state_rect(&self) -> Rect {
        Rect::new(
            self.x - BUTTON_RECT_STATE_SIZE as i32 / 2,
            self.y - BUTTON_RECT_STATE_SIZE as i32 / 2,
            BUTTON_RECT_STATE_SIZE,
            BUTTON_RECT_STATE_SIZE,
        )
    }
}

//return the percentage from 0 to 100 in a value with a int.
pub fn percentage_from_int(value: &i32, max: &i32) -> u8 {
    if *value < 1 {
        return 0;
    }
    let over = *value * 100 / *max; //The over value is used to check if is bigger than 100;
    if over <= 100 {
        over as u8
    } else {
        100
    }
}

//Takes a precentage from 0 to 100 and return the possible value.
pub fn int_from_percentage(value: &i32, percentage: &u8) -> i32 {
    *value * *percentage as i32 / 100
}

//Return a point based mainly on the angle and the distance. Remember a angle of 0.5 = 45 degrees clock wise.
//Plus the starting direction is ->.
pub fn angle_point<P: Into<Point>> (point: P, mut angle: f32, distance: f32) -> Point  {
    let point = point.into();
    angle = PI * (angle * 2.0);
    Point::new(
        point.x() + (distance * angle.cos()) as i32,
        point.y() + (distance * angle.sin()) as i32
    )
}

//Creates points for a basic geometry based on the vertices. For example, 3 vertices would give a triangle.
pub fn geometry<P: Into<Point>> (pos: P, vertices: u16, size: f32) -> Vec<Point> {
    let pos = pos.into();
    let mut edges: Vec<Point> = Vec::new();
    let angle_difference = 1.0 / vertices as f32;

    for i in 0..vertices { 
        edges.push(angle_point(pos, angle_difference * i as f32, size)); 
    }
    edges
}