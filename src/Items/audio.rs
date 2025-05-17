use std::ops::Deref;
use sdl3::audio::*;

use sdl3::render::FPoint;
use std::path::*;
use sdl3::event::Event;
use thebox::{Display, SdlContext, Write, PrimitiveNumber};
use thebox::{Slider, SliderType};

pub const NAME: &str = "Audio";
pub const ID: u8 = 6;

struct AudioBuffer {
    format: AudioFormat,
    audio_buffer: Vec<u8>,
    last_heard: Vec<i16>,
    index: usize,
}

impl AudioBuffer {
    pub fn new(audio_path: &PathBuf) -> AudioBuffer {
        let audio = AudioSpecWAV::load_wav(audio_path).unwrap();
        AudioBuffer {
            format: audio.format,
            audio_buffer: audio.buffer().to_owned(),
            last_heard: Vec::new(),
            index: 0,
        }
    }
    
    pub fn from_wav_spec(audio_wav: AudioSpecWAV) -> AudioBuffer {
        AudioBuffer {
            format: audio_wav.format,
            audio_buffer: audio_wav.buffer().to_owned(),
            last_heard: Vec::new(),
            index: 0,
        }
    }
}

impl AudioCallback<i16> for AudioBuffer {
    fn callback(&mut self, stream: &mut AudioStream, requested: i32) {
        let requested_index = self.index + requested as usize * 2;
        if requested_index >= self.audio_buffer.len() {
            return;
        }
        let requested_buffer: &[u8] = &self.audio_buffer[self.index..requested_index];
        stream.put_data(&requested_buffer).unwrap();
        
        let mut buffer: Vec<i16> = Vec::with_capacity(requested as usize);
        for bytes in requested_buffer.chunks(2) {
            match self.format {
                AudioFormat::S16LE => buffer.push(i16::from_le_bytes([bytes[0], bytes[1]])),
                AudioFormat::S16BE => buffer.push(i16::from_be_bytes([bytes[0], bytes[1]])),
                _ => panic!("UNSUPPORTED FORMAT! (THE DEV WAS TO LAZY TO IMPLEMENT THE OTHERS!)")
            }
        }
        self.last_heard = buffer;
        self.index += requested as usize * 2;
    }
}

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, _write: &Write) {
    let mut audio_path: PathBuf = PathBuf::from(thebox::get_assets_path());
    audio_path.push("audio_demo.wav");

    let audio_data = AudioSpecWAV::load_wav(&audio_path).unwrap();

    let desired_spec = AudioSpec {
        freq: Some(audio_data.freq),
        channels: Some(audio_data.channels as i32),
        format: Some(audio_data.format),
    };

    let queue_device: AudioDevice = match sdl_context
        .audio_subsystem
        .open_playback_device(&desired_spec)
    {
        Ok(audio_device) => audio_device,
        Err(damn) => panic!("{}", damn),
    };
    
    queue_device.resume();
    
    let mut queue = initialize_audio_stream_callback(&queue_device, &audio_path);
    
    queue.resume().unwrap();
    let samples_amount: u32 = 800;
    let limit: i16 = 32000;
    let mut sliders = create_sliders(display, -limit, limit, 0, display.height(), samples_amount);
    'repeat: loop {
        display.canvas.set_draw_color(thebox::DEFAULT_CLEAR_COLOR);
        display.canvas.clear();

        copy_buffer(&mut sliders, &queue.lock().unwrap().last_heard);
        
        
        for event in sdl_context.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'repeat;
                },
                Event::DropFile { filename, .. } => {
                    println!("{:?}", filename);
                    let audio = AudioSpecWAV::load_wav(filename).unwrap();
                    let mut current_audio = queue.lock().unwrap();
                    current_audio.audio_buffer = audio.buffer().to_owned();
                    current_audio.index = 0;
                }
                _ => {}
            }
        }

        for code in sdl_context.event_pump.keyboard_state().scancodes() {
            use sdl3::keyboard::Scancode::*;
            if code.1 {
                match code.0 {
                    Escape => sdl_context.send_quit().unwrap(),
                    _ => {}
                }
            } else {
                //match code.0 {
                //    _ => {}
                //}
            }
        }

        let sliders_points: Vec<FPoint> = sliders.iter().map(|slider| slider.pivot_f()).collect();
        display.canvas.set_draw_color(thebox::DEFAULT_COLOR);
        display.canvas.draw_lines(sliders_points.as_slice()).unwrap();
        display.canvas.present();
        display.sleep()
    }
    queue.pause().unwrap()
}

fn initialize_audio_stream_callback(device: &AudioDevice, audio_path: &PathBuf) 
    -> AudioStreamWithCallback<AudioBuffer> {
    let audio_data = AudioSpecWAV::load_wav(&audio_path).unwrap();
    let desired_spec = AudioSpec {
        freq: Some(audio_data.freq),
        channels: Some(audio_data.channels as i32),
        format: Some(audio_data.format),
    };
    device.open_playback_stream_with_callback(
        &desired_spec, 
        AudioBuffer::from_wav_spec(audio_data)
    ).unwrap()
}

fn create_sliders<Type: PrimitiveNumber>(
    display: &Display,
    min: Type,
    max: Type,
    value: Type,
    length: u32,
    quantity: u32,
) -> Vec<Slider<Type>> {
    let mut sliders: Vec<Slider<Type>> = Vec::with_capacity(quantity as usize);
    let width_distance = (display.width_f()) / (quantity - 1) as f32;
    let height = display.height_center_f();

    for index in 0..quantity {
        let mut slider = Slider::new(
            min,
            max,
            (width_distance * index as f32) as i32,
            height as i32,
            length,
            SliderType::SliderVertical,
        );
        slider.set_value_limited(value);
        sliders.push(slider);
    }
    sliders
}

fn copy_buffer<Type: PrimitiveNumber>(sliders: &mut Vec<Slider<Type>>, buffer: &[Type]) {
    if buffer.is_empty() {
        return;
    }
    for slider in sliders.iter_mut().enumerate() {
        slider.1.set_value_limited(buffer[slider.0])
    }
}
