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

const SLIDER_PIVOT_COLOR: Color = Color::RGB(120, 120, 120);
const SLIDER_PIVOT_SIZE: u32 = 14;
const SLIDER_BAR_SIZE: u32 = 8;

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
    pub value: u16,
    pub min: u16,
    pub max: u16,
    pub x: i32,
    pub y: i32,
    pub length: u32,
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

    //Draws a Slider on screen according to its values.
    pub fn draw_slider (&mut self, slider: &Slider) -> Result<(), String> {
        self.canvas.fill_rect(slider.bar_rect())?;
        self.canvas.set_draw_color(SLIDER_PIVOT_COLOR);
        self.canvas.fill_rect(slider.pivot_rect())?;
        self.canvas.set_draw_color(DEFAULT_COLOR);
        Ok(())
    }

    //Same as above, but is drawn with a predefined color.
    pub fn draw_slider_cl (&mut self, slider: &Slider, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(slider.bar_rect())?;
        self.canvas.set_draw_color(SLIDER_PIVOT_COLOR);
        self.canvas.fill_rect(slider.pivot_rect())?;
        self.canvas.set_draw_color(DEFAULT_COLOR);
        Ok(())
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

impl Slider {
    pub fn new(
        min: u16,
        max: u16,
        x: i32,
        y: i32,
        length: u32,
        slider_type: SliderType
    )  -> Slider {
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

    pub fn set_value(&mut self, value: u16) {
        self.value = value;
    }

    //Recommended for controled values.
    pub fn set_value_limited(&mut self, value: u16) {
        if value < self.min { self.value = self.min }
        else if value > self.max { self.value = self.max }
        else { self.value = value };
    }

    //Returns how filled is the slider.
    pub fn percentage(&self) -> f32 {
        self.value as f32 / self.max as f32
    }

    //Calculates and returns the position of the pivot.
    pub fn pivot(&self) -> Point {
        match &self.slider_type {
            SliderType::SliderHorizontal => { Point::new((self.x as f32 * self.percentage()) as i32, self.y) },
            SliderType::SliderVertical => { Point::new(self.x, (self.y as f32 * self.percentage()) as i32) }
        }
    }

    //Returns the Rect of the pivot.
    pub fn pivot_rect(&self) -> Rect {
        let half_size: i32 = (SLIDER_PIVOT_SIZE / 2) as i32;
        match self.slider_type {
            SliderType::SliderHorizontal => { 
                Rect::new(
                    self.x + (self.length as f32 * self.percentage()) as i32 - half_size, 
                    self.y - half_size,
                    SLIDER_PIVOT_SIZE,
                    SLIDER_PIVOT_SIZE
                )
            },
            SliderType::SliderVertical => { 
                Rect::new(
                    self.x - half_size,
                    self.y + (self.length as f32 * self.percentage()) as i32 - half_size,
                    SLIDER_PIVOT_SIZE,
                    SLIDER_PIVOT_SIZE
                ) 
            }
        }
    }

    //Returns the Rect of the Bar.
    pub fn bar_rect(&self) -> Rect {
        match self.slider_type {
            SliderType::SliderHorizontal => {
                Rect::new(
                    self.x,
                    self.y - SLIDER_BAR_SIZE as i32 / 2,
                    self.length,
                    SLIDER_BAR_SIZE
                )
            }, 
            SliderType::SliderVertical => {
                Rect::new(
                    self.x - SLIDER_BAR_SIZE as i32 / 2,
                    self.y,
                    SLIDER_BAR_SIZE,
                    self.length
                )
            }
        }
    }

    pub fn update_from_pos<P: Into<Point>>(&mut self, point: P) {
        let point = point.into();
        let distance: i32;

        match self.slider_type {
            SliderType::SliderHorizontal => { distance = point.x() - self.x },
            SliderType::SliderVertical => { distance = point.y() - self.y }
        }
        self.set_value_limited(
            int_from_percentage(
                &(self.max as i32),
                &percentage_from_int(&distance, &(self.length as i32))
            ).try_into().unwrap()    
        );
    }
}

//return the percentage from 0 to 100 in a value with a int.
//WARNING: DON'T USE FLOAT IN THIS!
pub fn percentage_from_int(value: &i32, max: &i32) -> u8
{
    if *value < 1 { return 0; }
    let over = *value * 100 / *max; //The over value is used to check if is bigger than 100;
    if over <= 100 { over as u8 }
    else { 100 }
}

//Takes a precentage from 0 to 100 and return the possible value.
pub fn int_from_percentage(value: &i32, percentage: &u8) -> i32
{
    *value * *percentage as i32 / 100
}