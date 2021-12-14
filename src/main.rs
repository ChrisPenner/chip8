#![feature(bigint_helper_methods)]
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

mod font;
mod graphics;
mod ops;

use ops::Compy;

const PIXEL_SIZE: usize = 10;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let width = graphics::WIDTH * PIXEL_SIZE;
    let height = graphics::HEIGHT * PIXEL_SIZE;
    let window = video_subsystem
        .window("rust-sdl2 demo", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut compy = Compy::new();
    // compy.load_rom("./roms/c8_test.c8");
    // compy.load_rom("./roms/maze_alt.ch8");
    // compy.load_rom("./roms/Particle Demo [zeroZshadow, 2008].ch8");
    compy.load_rom("./roms/programs/Random Number Test [Matthew Mikolay, 2010].ch8");
    // compy.load_rom("./roms/games/ZeroPong [zeroZshadow, 2007].ch8");

    'running: loop {
        compy.single_cycle();

        i = (i + 1) % 255;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => compy.set_key_state(true, keycode),
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => compy.set_key_state(false, keycode),
                _ => {}
            }
        }

        let mut clear_i = None;
        if compy.draw_ops.len() != 0 {
            for (i, op) in compy.draw_ops.iter().enumerate() {
                match op {
                    ops::DrawOp::Clear => {
                        canvas.set_draw_color(Color::RGB(0, 0, 0));
                        canvas.clear();
                        clear_i = Some(i);
                    }
                    ops::DrawOp::Sprite(v) => {
                        for (p, b) in v {
                            draw_pixel(&mut canvas, p, *b)
                        }
                    } // draw(&compy, &mut canvas);
                      // compy.draw_flag = false;
                }
            }
            if let Some(clear_index) = clear_i {
                compy.draw_ops = compy.draw_ops.split_off(clear_index);
            }
            canvas.present();
        }
    }
}

pub fn draw_pixel(canvas: &mut Canvas<Window>, p: &sdl2::rect::Point, b: bool) {
    if b {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
    } else {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
    }
    let rect = Rect::new(
        p.x * PIXEL_SIZE as i32,
        p.y * PIXEL_SIZE as i32,
        PIXEL_SIZE as u32,
        PIXEL_SIZE as u32,
    );
    canvas.fill_rect(rect).unwrap();
}
