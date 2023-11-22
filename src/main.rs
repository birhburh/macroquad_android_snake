use macroquad::math::Vec2;
use macroquad::prelude::*;

use std::collections::LinkedList;

const SQUARES: i16 = 16;

type Point = (i16, i16);

struct Snake {
    head: Point,
    body: LinkedList<Point>,
    dir: Point,
}

pub fn draw_segment(x: f32, y: f32, radius: f32, rotation: f32, color: Color) {
    let rot = rotation.to_radians();
    let mut prev = Default::default();
    for i in 0..(5 + 1) {
        let rx = ((i as f32 * std::f32::consts::PI * 2.) / 20. + rot).cos();
        let ry = ((i as f32 * std::f32::consts::PI * 2.) / 20. + rot).sin();

        if i != 0 {
            draw_triangle(
                Vec2::new(x, y),
                prev,
                Vec2::new(x + radius * rx, y + radius * ry),
                if i % 2 == 0 { color } else { MAGENTA },
            );
        }
        prev = Vec2::new(x + radius * rx, y + radius * ry);
    }
}

fn rotate(p: Vec2, c: Vec2, angle: f32) -> Vec2 {
    let angle = angle.to_radians();
    Vec2::new(
        angle.cos() * (p.x - c.x) - angle.sin() * (p.y - c.y) + c.x,
        angle.sin() * (p.x - c.x) + angle.cos() * (p.y - c.y) + c.y,
    )
}

#[macroquad::main("Snake")]
async fn main() {
    let mut snake = Snake {
        head: (0, 0),
        dir: (1, 0),
        body: LinkedList::new(),
    };
    let mut fruit: Point = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
    let mut score = 0;
    let mut speed = 0.5;
    let mut last_update = get_time();
    let mut game_over = false;
    let mut touched;
    let mut touch_start = Default::default();
    let mut up_touch = false;
    let mut down_touch = false;
    let mut left_touch = false;
    let mut right_touch = false;

    let up = (0, -1);
    let down = (0, 1);
    let right = (1, 0);
    let left = (-1, 0);

    loop {
        touched = false;
        for touch in touches().iter().take(1) {
            match touch.phase {
                TouchPhase::Ended => {
                    touched = true;
                }
                _ => (),
            };
        }
        if !game_over {
            if right_touch && snake.dir != left {
                snake.dir = right;
            } else if left_touch && snake.dir != right {
                snake.dir = left;
            } else if up_touch && snake.dir != down {
                snake.dir = up;
            } else if down_touch && snake.dir != up {
                snake.dir = down;
            }
            up_touch = false;
            down_touch = false;
            left_touch = false;
            right_touch = false;

            if get_time() - last_update > speed {
                last_update = get_time();
                snake.body.push_front(snake.head);
                snake.head = (snake.head.0 + snake.dir.0, snake.head.1 + snake.dir.1);
                if snake.head == fruit {
                    fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                    score += 100;
                    speed *= 0.9;
                } else {
                    snake.body.pop_back();
                }
                if snake.head.0 < 0
                    || snake.head.1 < 0
                    || snake.head.0 >= SQUARES
                    || snake.head.1 >= SQUARES
                {
                    game_over = true;
                }
                for (x, y) in &snake.body {
                    if *x == snake.head.0 && *y == snake.head.1 {
                        game_over = true;
                    }
                }
            }
        }
        if !game_over {
            clear_background(LIGHTGRAY);

            let game_size = screen_width().min(screen_height());
            let offset_x = (screen_width() - game_size) / 2. + 10.;
            let offset_y = (screen_height() - game_size) / 2. + 10.;
            let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

            draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

            for i in 1..SQUARES {
                draw_line(
                    offset_x,
                    offset_y + sq_size * i as f32,
                    screen_width() - offset_x,
                    offset_y + sq_size * i as f32,
                    2.,
                    LIGHTGRAY,
                );
            }

            for i in 1..SQUARES {
                draw_line(
                    offset_x + sq_size * i as f32,
                    offset_y,
                    offset_x + sq_size * i as f32,
                    screen_height() - offset_y,
                    2.,
                    LIGHTGRAY,
                );
            }

            draw_rectangle(
                offset_x + snake.head.0 as f32 * sq_size,
                offset_y + snake.head.1 as f32 * sq_size,
                sq_size,
                sq_size,
                DARKGREEN,
            );

            for (x, y) in &snake.body {
                draw_rectangle(
                    offset_x + *x as f32 * sq_size,
                    offset_y + *y as f32 * sq_size,
                    sq_size,
                    sq_size,
                    LIME,
                );
            }

            draw_rectangle(
                offset_x + fruit.0 as f32 * sq_size,
                offset_y + fruit.1 as f32 * sq_size,
                sq_size,
                sq_size,
                GOLD,
            );

            draw_text(
                format!("SCORE: {}", score).as_str(),
                10.,
                10.,
                20.,
                DARKGRAY,
            );
        } else {
            clear_background(WHITE);
            let text = "Game Over. Touch screen to play again.";
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                font_size,
                DARKGRAY,
            );

            if touched {
                snake = Snake {
                    head: (0, 0),
                    dir: (1, 0),
                    body: LinkedList::new(),
                };
                fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                score = 0;
                speed = 0.5;
                last_update = get_time();
                game_over = false;
                up_touch = false;
                down_touch = false;
                left_touch = false;
                right_touch = false;
            }
        }
        for touch in touches().iter().take(1) {
            let (fill_color, size) = match touch.phase {
                TouchPhase::Started => {
                    touch_start = touch.position;
                    (GREEN, 80.0)
                }
                TouchPhase::Stationary => (RED, 60.0),
                TouchPhase::Moved => (ORANGE, 60.0),
                TouchPhase::Ended => (BLUE, 80.0),

                TouchPhase::Cancelled => (BLACK, 80.0),
            };
            draw_line(
                touch_start.x,
                touch_start.y,
                touch.position.x,
                touch.position.y,
                2.,
                fill_color,
            );
            let line_end = Vec2::new(touch_start.x, touch_start.y - size);
            for i in 0..4 {
                let new_end = rotate(line_end, touch_start, 90.0 * i as f32 + 45.0);
                draw_line(
                    touch_start.x,
                    touch_start.y,
                    new_end.x,
                    new_end.y,
                    2.,
                    fill_color,
                );
            }
            let angle = (touch_start.y - touch.position.y)
                .atan2(touch_start.x - touch.position.x)
                .to_degrees();
            let seg_ang;
            if touch_start != touch.position {
                match angle {
                    x if x >= 0. && x < 45. => {
                        left_touch = true;
                        seg_ang = 135.;
                    }
                    x if x >= 45. && x < 135. => {
                        up_touch = true;
                        seg_ang = 225.;
                    }
                    x if x >= 135. && x <= 180. => {
                        right_touch = true;
                        seg_ang = 315.;
                    }
                    x if x <= -135. && x >= -180. => {
                        right_touch = true;
                        seg_ang = 315.;
                    }
                    x if x <= -45. && x > -135. => {
                        down_touch = true;
                        seg_ang = 45.;
                    }
                    x if x < 0. && x > -45. => {
                        left_touch = true;
                        seg_ang = 135.;
                    }
                    _ => panic!("Wrong angle! How did you even make this?!"),
                }
                draw_segment(touch_start.x, touch_start.y, size, seg_ang, SKYBLUE);
            }
            draw_circle_lines(touch_start.x, touch_start.y, size, 2., fill_color);
            draw_circle(touch.position.x, touch.position.y, size, fill_color);
            draw_text(
                format!("ANGLE: {}", angle).as_str(),
                10.,
                30.,
                20.,
                DARKGRAY,
            );
        }
        next_frame().await
    }
}
