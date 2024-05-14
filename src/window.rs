use sdl2::pixels::Color;
use sdl2::ttf;

use sdl2::Sdl;
use sdl2::{EventPump, VideoSubsystem};

use sdl2::render::*;
use sdl2::video::WindowContext;
use sdl2::gfx::framerate::FPSManager;

pub struct SdlContext {
	pub sdl2: Sdl,
	pub event_pump: EventPump,
	pub video_subsystem: VideoSubsystem,
}

pub struct Display {
	pub canvas: WindowCanvas,
	pub texture_creator: TextureCreator <WindowContext>,
	pub fps_manager: FPSManager,
}

pub struct Write<'t, 'f> {
	pub ttf : &'t ttf::Sdl2TtfContext,
	pub font : ttf::Font<'t, 'f>,
	pub color : Color,
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

		let window = video_subsystem.window("BOX", width, height)
			.opengl()        
        	.position_centered()
        	.build().unwrap();

		let canvas = window.into_canvas()
			.present_vsync()
        	.build().unwrap();
        let texture_creator = canvas.texture_creator();
        let fps_manager = FPSManager::new();
        Display {
        	canvas,
        	texture_creator,
        	fps_manager,
        }
	}

	//Draws a text with the given string.
	pub fn draw_text (&mut self, write: &Write, x: i32, y: i32,  string: &str, size: u32) {
    	let texture = write.create_text(&self.texture_creator, string);
    	let string_len : u32 = string.len().try_into().unwrap();

    	let area = sdl2::rect::Rect::new(x, y, string_len * size, size * 2);
    	let _ = self.canvas.copy(&texture, None, area);
	}

	//Same as above but cetered.
	pub fn draw_text_centered (&mut self, write: &Write, x: i32, y: i32, string: &str, size: u32) {
    	let texture = write.create_text(&self.texture_creator, string);
    	let string_len : u32 = string.len().try_into().unwrap();

    	let middle : i32 = ((string_len * size) / 2).try_into().unwrap();	//I hate this middle variable.

    	let area = sdl2::rect::Rect::new(x - middle, y, string_len * size, size * 2);
    	let _ = self.canvas.copy(&texture, None, area);
	}
}

impl Write<'_, '_> {
	pub fn init_write<'t, 'f>(ttf: &'t ttf::Sdl2TtfContext, color: Color) -> Write<'t, 'f> {
		let font: ttf::Font<'t, 'f> = ttf.load_font("./src/main_assets/Fixedsys.ttf", 32).expect("COULD NOT FIND FONT!");
		let color = color;
		Write {
			ttf,
			font,
			color,
		}
	}

	pub fn create_text<'b>(& self, texture_creator: &'b TextureCreator<WindowContext>, string: &str) -> Texture<'b> {
		self.font.render(string).solid(self.color).unwrap().as_texture(texture_creator).unwrap()
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