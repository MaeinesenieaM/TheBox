use std::io::Read;
//use sdl2::pixels::Color;
use sdl3::keyboard::*;
use sdl3::audio::*;

use std::path::*;
use sdl3::render::FPoint;
use thebox::{Display, Write, SdlContext, Draw, PrimitiveNumber};
use thebox::{Slider, SliderType};

pub const NAME: &str = "Audio";
pub const ID: u8 = 6;

pub fn start(display: &mut Display, sdl_context: &mut SdlContext, write: &Write) {

    //let mut volume_slider = Slider::new(-0.5f32, 0.5f32, 200, 300, 256, SliderType::SliderVertical);

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
    println!("{:?}", audio_data.format);

    let mut sliders = create_sliders(
        display,
        -1f32,
        1f32,
        0f32,
        128,
        16
    );
    
    'repeat: loop {
        display.canvas.set_draw_color(thebox::DEFAULT_CLEAR_COLOR);
        display.canvas.clear();

        let mut buff: Vec<f32> = vec!(0f32; 2);
        //queue.read_f32_samples(&mut buff).unwrap();
        //queue.read_to_end(&mut buff).unwrap();
        println!("{:?}", queue.read_f32_samples(&mut buff).unwrap());
        println!("{:?}", buff);

        let mut slider_buff: Vec<f32> = vec!(0f32; 8);
        
        sliders.last_mut().unwrap().set_value_limited(buff[0]);
        
        //volume_slider.set_value_limited(buff[0]);

        let keyboard: KeyboardState = KeyboardState::new(&sdl_context.event_pump);
        
        if keyboard.is_scancode_pressed(Scancode::Escape) {let _ = sdl_context.send_quit();}
        if sdl_context.check_quit() {break 'repeat}

        //volume_slider.draw_cl(display, COLOR_GREEN).unwrap();
        
        //for slider in sliders.iter() {
        //    slider.draw_cl(display, thebox::COLOR_RED).unwrap()
        //}
        
        let sliders_points: Vec<FPoint> = sliders.iter().map(|slider| slider.pivot_f()).collect();
        display.canvas.set_draw_color(thebox::DEFAULT_COLOR);
        display.canvas.draw_lines(sliders_points.as_slice()).unwrap();
        
        audio_message.draw(display).unwrap();

        display.canvas.present();
        display.sleep()
    }
}

fn create_sliders<Type: PrimitiveNumber>(
    display: &Display,
    min: Type,
    max: Type,
    value: Type,
    length: u32,
    quantity: u32
) -> Vec<Slider<Type>> {
    let mut sliders: Vec<Slider<Type>> = Vec::with_capacity(quantity as usize);
    
    let offset = display.width() / (quantity * 2);
    
    let width_distance = (display.width()) / quantity;
    let height = display.height_center();
    
    for index in 0..quantity {
        let mut slider = Slider::new(
            min,
            max,
            (offset + width_distance * index) as i32,
            height as i32,
            length,
            SliderType::SliderVertical
        );
        slider.set_value_limited(value);
        sliders.push(slider);
    }
    sliders
}

//fn push_sliders<Type: PrimitiveNumber>(sliders: &mut Vec<Slider<Type>>, value: Type) {
    //for slider in sliders.iter_mut().rev().skip(1) {
    //    slider
    //}
//    sliders.truncate()
//    
//}

fn copy_buffer<Type: PrimitiveNumber>(sliders: &mut Vec<Slider<Type>>, buffer: &[Type]) {
    for slider in sliders.iter_mut().enumerate() {
        slider.1.set_value_limited(buffer[slider.0])
    }
}

//fn to_volume(sample: &[f32]) -> f32 { 
//    (sample[0] * sample[0]).sqrt() + (sample[1] * sample[1]).sqrt() / 2f32
//}