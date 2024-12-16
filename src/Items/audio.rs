use sdl2::event::Event;
//use sdl2::pixels::Color;
use sdl2::keyboard::*;
use sdl2::audio::*;

use std::path::*;
use thebox::{Display, Write, SdlContext};

pub const NAME: &str = "Audio";
pub const ID: u8 = 6;

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &mut Write) {

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

    //This Vec stores the buffer of the sound for it to play.
    let mut sound_buffer: Vec<i16> = Vec::new();

    //This acts as mini storage, that when full mixes the two bytes [u8] into one byte [u16] and resets.
    let mut byte_buffer: Vec<u8> = Vec::with_capacity(2);

    for data in audio_spec.buffer() {
        byte_buffer.push(*data);
        if byte_buffer.len() == 2 {
            sound_buffer.push(i16::from_le_bytes([byte_buffer[0], byte_buffer[1]]));
            byte_buffer.clear();
        }
    }

    let _ = queue.queue_audio(&sound_buffer);

    //mini memory management :)
    drop(sound_buffer);
    drop(audio_spec);

    queue.resume();

    'repeat: loop {
        display.canvas.clear();

        for event in sdl_context.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'repeat,
                _ => {}
            }
        }

        display.draw_text_centered(
            &write,
            400,
            550,
            "I Like Bells.",
            16,
        );

        display.canvas.present();
    }
}
