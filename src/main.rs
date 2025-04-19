//possible required code.
//use fastrand;
use std::time::{Duration, Instant};

//use sdl2::pixels::Color;

use sdl3::event::Event;
use sdl3::keyboard::*;

use thebox::*;

pub mod items;

fn main() {
    println!("The box shall open, once again...");

    let mut sdl_context = SdlContext::init_context();
    let mut display = Display::init_display(&sdl_context.video_subsystem, 800, 600);
    
    let ttf = sdl3::ttf::init().unwrap();
    let texture_creator = display.canvas.texture_creator();
    let write = Write::init_write(&ttf, &texture_creator, "Fixedsys.ttf");
    
    let mut frames = 0;
    let mut temp_frames = 0;
    let mut instant = Instant::now();

    let mut count: i32 = 1;
    let mut enter: u16 = 0;

    let mut fps_label = Label::new(400, 0, 8, &write, None);
    let mut count_label = Label::new(400, 300, 16, &write, Some(count.to_string()));
    let mut item_label = Label::new(400, 332, 16, &write, None);
    
    'running: loop {
        for event in sdl_context.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    enter = 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    count -= 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    count += 1;
                }
                _ => {}
            }
        }

        if enter == 1 {
            items::select_item(count, &mut display, &mut sdl_context, &write);
            enter = 0;
        }

        //basic framerate counter.
        if instant.elapsed() >= Duration::from_secs(1) {
            temp_frames = frames;
            frames = 0;
            instant = Instant::now();
        }

        display.canvas.set_draw_color(DEFAULT_CLEAR_COLOR);
        display.canvas.clear();

        item_label.update_text(items::name_item(count).to_string());
        count_label.update_text(count.to_string());
        fps_label.update_text(temp_frames.to_string());
        
        let _ = item_label.draw(&mut display);
        let _ = count_label.draw(&mut display);
        let _ = fps_label.draw(&mut display);

        frames += 1;
        display.canvas.present();
        display.sleep()
    }
}
