//possible required code.
//use fastrand;

use sdl2::pixels::Color;
//use sdl2::render::*;

use sdl2::event::Event;
//use sdl2::keyboard::*;
//use sdl2::rect::*;

fn main() {
    println!("The box shall open, once again...");

    let sdl2 = sdl2::init().unwrap();
    let video_subsystem = sdl2.video().unwrap();

    let window = video_subsystem.window("Box", 800, 600)
        .position_centered()
        .build().unwrap();
    let mut canvas = window.into_canvas()
        .present_vsync()
        .build().unwrap();

    let mut event_pump = sdl2.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(Color::RGB(20, 20, 20));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event{
                Event::Quit {..} => { break 'running }, 
                _ => {}
            }
        }

        canvas.present();
    }
}

/*
The MIT License (MIT)

Copyright (c) 2013 Mozilla Foundation

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/