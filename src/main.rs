extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::time::Duration;

const FPS: u32 = 60;
const MAX_BALL_SPEED: f32 = 600f32;
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const RECT_SIZE: u32 = 20;
const PLAYER_WIDTH: u32 = RECT_SIZE * 6;
const PLAYER_HEIGHT: u32 = RECT_SIZE;

struct Block {
    rect: Rect,
    is_dead: bool,
    color: Color,
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Breakout", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut ball_vel_x: f32 = 1f32;
    let mut ball_vel_y: f32 = -1f32;

    let mut player_vel_x: f32 = 0f32;

    let delta_time: f32 = 1f32 / FPS as f32;

    let player_first_y = WINDOW_HEIGHT - 100;

    let mut ball = Rect::new(
        (WINDOW_WIDTH - RECT_SIZE as i32) / 2,
        player_first_y - 100,
        RECT_SIZE,
        RECT_SIZE,
    );
    // let mut ball_speed: f32 = 300f32;
    let mut ball_speed: f32 = 400f32;

    let mut player = Rect::new(
        (WINDOW_WIDTH - PLAYER_WIDTH as i32) / 2,
        player_first_y,
        PLAYER_WIDTH,
        PLAYER_HEIGHT,
    );

    let mut blocks: Vec<Block> = Vec::new();

    let starting_x = 0;
    let mut block_x = starting_x;
    let mut starting_y = 10;
    let block_x_gap = 70;
    let block_y_gap = 30;

    let colors: Vec<Color> = vec![Color::RGB(200, 200, 200), Color::RGB(150, 150, 150)];

    for i in 0..50 {
        let block = Rect::new(block_x, starting_y, 80, RECT_SIZE);
        blocks.push(Block {
            rect: block,
            is_dead: false,
            color: colors[i % colors.len()],
        });

        let mut new_x = block_x + RECT_SIZE as i32 + block_x_gap;

        if new_x > WINDOW_WIDTH {
            new_x = starting_x;
            starting_y += block_y_gap;
        }

        block_x = new_x;
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(16, 16, 16));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    player_vel_x = -1f32;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    player_vel_x = 1f32;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    player_vel_x = 0f32;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    player_vel_x = 0f32;
                }
                _ => {}
            }
        }

        if blocks.iter().filter(|x| !x.is_dead).count() == 0 {
            println!("You won!");
            break;
        }

        canvas.set_draw_color(Color::RGB(50, 255, 50));
        _ = canvas.fill_rect(ball);

        canvas.set_draw_color(Color::RGB(255, 50, 50));
        _ = canvas.fill_rect(player);

        for block in blocks.iter_mut().filter(|x| !x.is_dead) {
            if ball.intersection(block.rect).is_some() {
                ball_vel_x = 1f32;
                ball_vel_y = 1f32;

                block.is_dead = true;
                continue;
            }

            canvas.set_draw_color(block.color);
            _ = canvas.fill_rect(block.rect);
        }

        let player_speed: f32 = ball_speed * 1.5;

        if ball.intersection(player).is_some() {
            if ball.y >= player.y {
                ball_vel_y = 1f32;
            } else {
                ball_vel_y = -1f32;
            }
        }

        if ball.x <= 0 {
            ball_vel_x = 1f32;
            ball_speed = increase_speed(ball_speed);
        } else if ball.y <= 0 {
            ball_vel_y = 1f32;
            ball_speed = increase_speed(ball_speed);
        } else if ball.x + RECT_SIZE as i32 >= WINDOW_WIDTH {
            ball_vel_x = -1f32;
            ball_speed = increase_speed(ball_speed);
        } else if ball.y + RECT_SIZE as i32 >= WINDOW_HEIGHT {
            ball_vel_y = -1f32;
            ball_speed = increase_speed(ball_speed);
        }

        ball.x += (ball_vel_x * ball_speed * delta_time) as i32;
        ball.y += (ball_vel_y * ball_speed * delta_time) as i32;
        player.x += (player_vel_x * player_speed * delta_time) as i32;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}

fn increase_speed(ball_speed: f32) -> f32 {
    let sp: f32 = 5f32;

    let new_speed: f32 = if ball_speed + sp < MAX_BALL_SPEED {
        ball_speed + sp
    } else {
        MAX_BALL_SPEED
    };

    new_speed
}
