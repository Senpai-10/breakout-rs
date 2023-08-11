extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::time::Duration;

const FPS: u32 = 60;
const SPEED: f32 = 220f32;
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const RECT_SIZE: u32 = 30;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Breakout", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut x: f32 = 100f32;
    let mut y: f32 = 100f32;
    let delta_time: f32 = 1f32 / FPS as f32;
    let mut ball_vel_x: f32 = 1f32;
    let mut ball_vel_y: f32 = 1f32;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(16, 16, 16));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        if x <= 0f32 {
            ball_vel_x = 1f32;
        }

        if y <= 0f32 {
            ball_vel_y = 1f32;
        }

        if x + RECT_SIZE as f32 >= WINDOW_WIDTH as f32 {
            ball_vel_x = -1f32;
        }

        if y + RECT_SIZE as f32 >= WINDOW_HEIGHT as f32 {
            ball_vel_y = -1f32;
        }

        x += ball_vel_x * SPEED * delta_time;
        y += ball_vel_y * SPEED * delta_time;

        let ball = Rect::new(x as i32, y as i32, RECT_SIZE, RECT_SIZE);

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        _ = canvas.fill_rect(ball);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}
