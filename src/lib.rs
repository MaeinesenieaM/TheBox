use sdl2::pixels::Color;
use sdl2::ttf;

use sdl2::Sdl;
use sdl2::{EventPump, VideoSubsystem};

use sdl2::gfx::framerate::FPSManager;
use sdl2::render::*;
use sdl2::video::WindowContext;
use sdl2::rect::*;

pub const DEFAULT_COLOR: Color = Color::RGB(210, 210, 220);
pub const DEFAULT_CLEAR_COLOR: Color = Color::RGB(20, 20, 20);

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
pub struct Slider<T> {
    pub value: T,
    pub min: T,
    pub max: T,
    pub x: i32,
    pub y: i32,
    pub length: i32,
    pub slider_type: SliderType,
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

    pub fn draw_slider<T: Copy + std::cmp::PartialOrd> (&mut self, slider: &Slider<T>) -> Result<(), String>
    where f32: 
        From<T> 
    {
        let pivot: Point = slider.pivot();
        match &slider.slider_type {
            SliderType::SliderHorizontal => {
                self.canvas.fill_rect(Rect::new(slider.x, slider.y - 10, slider.length as u32, 20))?;
                self.canvas.fill_rect(Rect::new(pivot.x, pivot.y, 30, 30))?;
                Ok(())
            }, 
            SliderType::SliderVertical => {
                self.canvas.fill_rect(Rect::new(slider.x - 10, slider.y, 20, slider.length as u32))?;
                self.canvas.fill_rect(Rect::new(pivot.x, pivot.y, 30, 30))?;
                Ok(()) 
            }
        }
    }

}

impl Write<'_, '_> {
    pub fn init_write<'t, 'f>(ttf: &'t ttf::Sdl2TtfContext, color: Color) -> Write<'t, 'f> {
        let font = match ttf.load_font("./src/main_assets/Fixedsys.ttf", 32) {
            Ok(font_src) => { font_src },
            Err(_) => match ttf.load_font("./main_assets/Fixedsys.ttf", 32) {
                Ok(font_ass) => { font_ass },
                Err(damn) => { panic!("ERROR: {}", &damn) }
            }
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

impl<T: Copy + std::cmp::PartialOrd> Slider<T> where f32: From<T> {
    pub fn new(
        min: T,
        max: T,
        x: i32,
        y: i32,
        length: i32,
        slider_type: SliderType
    )  -> Slider<T> {
        Slider {
            min,
            max,
            value: min,
            x,
            y,
            length,
            slider_type
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn length(&self) -> i32 {
        self.length
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }

    //Recommended for controled values.
    pub fn set_value_limited(&mut self, value: T) {
        if value < self.min { self.value = self.min }
        else if value > self.max { self.value = self.max }
        else { self.value = value };
    }

    //Returns how filled is the slider.
    pub fn percentage(&self) -> f32 {
        f32::from(self.value) / f32::from(self.max)
    }

    //Calculates and returns the position of the pivot.
    pub fn pivot(&self) -> Point {
        match &self.slider_type {
            SliderType::SliderHorizontal => { Point::new((self.x as f32 * self.percentage()) as i32, self.y) },
            SliderType::SliderVertical => { Point::new(self.x, (self.y as f32 * self.percentage()) as i32) }
        }
    }
}