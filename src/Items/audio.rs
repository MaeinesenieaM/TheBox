use std::any::Any;
use std::io::Read;
//use sdl2::pixels::Color;
use sdl3::keyboard::*;
use sdl3::audio::*;

use std::path::*;
use thebox::{Display, Write, SdlContext, Draw};

pub const NAME: &str = "Audio";
pub const ID: u8 = 6;

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &Write) {

    let audio_message: thebox::Label = thebox::Label::new(
        400, 
        550,
        16,
        write,
        Some(String::from("I like bells"))
    );
    
    let mut audio_path: PathBuf = PathBuf::from(thebox::get_assets_path());
    audio_path.push("audio_demo.wav");

    let audio_data = AudioSpecWAV::load_wav(audio_path).unwrap();
    
    let desired_spec = AudioSpec {
        freq: Some(audio_data.freq),
        channels: Some(audio_data.channels as i32),
        format: Some(audio_data.format),
    };

    let audio_device = match sdl_context.audio_subsystem.open_playback_device(&desired_spec) {
        Ok(audio_device) => audio_device,
        Err(damn) => panic!("{}", damn)
    };

    let mut queue = audio_device.open_device_stream(Some(&desired_spec)).unwrap();

    //THIS IS SO MUCH BETTER THAN SDL2!
    queue.put_data(audio_data.buffer()).unwrap();
    queue.resume().unwrap();

    'repeat: loop {
        display.canvas.clear();
        
        let keyboard: KeyboardState = KeyboardState::new(&sdl_context.event_pump);
        
        if keyboard.is_scancode_pressed(Scancode::Escape) {let _ = sdl_context.send_quit();}
        if sdl_context.check_quit() {break 'repeat}

        let _ = audio_message.draw(display);

        display.canvas.present();
        display.sleep()
    }
}
