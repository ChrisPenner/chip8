use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

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

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut compy = Compy::new();

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
        // The rest of the game loop goes here...

        draw(&compy, &mut canvas);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw(compy: &Compy, canvas: &mut Canvas<Window>) {
    canvas.clear();
    let (lit, unlit) = compy.gfx.pixel_sets();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for p in lit {
        draw_pixel(canvas, p);
    }
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    for p in unlit {
        draw_pixel(canvas, p);
    }
    canvas.present();
}

pub fn draw_pixel(canvas: &mut Canvas<Window>, p: sdl2::rect::Point) {
    let rect = Rect::new(
        p.x * PIXEL_SIZE as i32,
        p.y * PIXEL_SIZE as i32,
        PIXEL_SIZE as u32,
        PIXEL_SIZE as u32,
    );
    canvas.fill_rect(rect).unwrap();
}
