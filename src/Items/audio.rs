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

    let audio_spec = AudioSpecWAV::load_wav(audio_path).unwrap();
    
    let desired_spec = AudioSpecDesired {
        freq: Some(audio_spec.freq),
        channels: Some(audio_spec.channels),
        samples: None
    };

    let queue = match sdl_context.audio_subsystem.open_queue(None, &desired_spec) {
        Ok(good) => good,
        Err(damn) => panic!("{}", damn)
    };
    
    for data in audio_spec.buffer().chunks(2) {
        queue.queue_audio(&[i16::from_le_bytes([data[0], data[1]])]).unwrap();
    }
    
    queue.resume();

    'repeat: loop {
        display.canvas.clear();
        
        let keyboard: KeyboardState = KeyboardState::new(&sdl_context.event_pump);
        
        if keyboard.is_scancode_pressed(Scancode::Escape) {let _ = sdl_context.send_quit();}
        if sdl_context.check_quit() {break 'repeat}

        let _ = audio_message.draw(display);

        display.canvas.present();
    }
}
