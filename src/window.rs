use sdl2::pixels::Color;
use sdl2::ttf;

use sdl2::Sdl;
use sdl2::EventPump;
use sdl2::video::Window;

use sdl2::render::*;
use sdl2::video::WindowContext;

pub struct SdlContext {
	pub sdl2: Sdl,
	pub event_pump: EventPump,
	pub window: Window,
}

pub struct Display {
	pub canvas: WindowCanvas,
	pub texture_creator: TextureCreator <WindowContext>,
}

impl SdlContext {
	pub fn init_window(width: u32, height: u32) -> SdlContext {
		let sdl2 = sdl2::init().unwrap();
		let event_pump = sdl2.event_pump().unwrap();

		let video_subsystem = sdl2.video().unwrap();
		let window = video_subsystem.window("BOX", width, height)
			.opengl()        
        	.position_centered()
        	.build().unwrap();

		SdlContext {
			sdl2,
			event_pump,
			window,
		}
	}
}

impl Display {
	pub fn init_display(window: Window) -> Display {
		let canvas = window.into_canvas()
			.present_vsync()
        	.build().unwrap();
        let texture_creator = canvas.texture_creator();
        Display {
        	canvas,
        	texture_creator,
        }
	}

	//Creates a texture of a text with the given string.
	pub fn create_text (&mut self, x: i32, y: i32, string: &str, size: u32) {
    	let ttf = ttf::init().unwrap();
    	let ttf_font = ttf.load_font("./src/main_assets/Fixedsys.ttf", 32).expect("COULD NOT FIND FONT!");

    	let texture = ttf_font.render(string).solid(Color::RGB(210, 210, 220)).unwrap().as_texture(&self.texture_creator).unwrap();

    	let string_len : u32 = string.len().try_into().unwrap();

    	let rectangle = sdl2::rect::Rect::new(x, y, string_len * size, size * 2);
    	let _ = self.canvas.copy(&texture, None, rectangle);
	}
}