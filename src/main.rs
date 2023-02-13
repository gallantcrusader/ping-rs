use cat_box::{get_keyboard_state, physics::check_for_collision, Game, Sprite};
use sdl2::keyboard::Scancode;
use std::time::Instant;

mod ball;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Still,
}
const CORNER: i32 = 37 / 2;

fn main() {
    let game = Game::new("pingPong", 500, 500);

    let mut ball = ball::Ball::new(250, 250);

    let paddle_bytes = include_bytes!("../paddle.png");
    let mut paddle = Sprite::from_bytes(paddle_bytes, 37, 250 - CORNER).unwrap();
    let mut paddle_enemy = Sprite::from_bytes(paddle_bytes, 500 - 37, 250 - CORNER).unwrap();

    let mut held = true;

    let mut dir = Direction::Still;
    let mut dir_enemy = Direction::Still;

    let mut now = Instant::now();
    
    // FPS COUNTER
    let mut now2 = Instant::now();
    let mut counter = 0;
    game.run(|ctx| {
        ctx.set_background_colour(0, 0, 0);

        let (x_player, y_player) = paddle.position().into();
        let (x_enemy, y_enemy) = paddle_enemy.position().into();

        let (x_ball, y_ball) = ball.sprite.position().into();

        let keys = get_keyboard_state(ctx).keys;
        for key in &keys {
            match key {
                Scancode::W | Scancode::Up => {
                    dir = Direction::Up;
                    held = true;
                }
                Scancode::S | Scancode::Down => {
                    dir = Direction::Down;
                    held = true;
                }
                _ => {
                    dir = Direction::Still;
                    held = false;
                }
            };
        }

        if y_enemy < y_ball {
            dir_enemy = Direction::Down;
        }
        if y_enemy > y_ball {
            dir_enemy = Direction::Up;
        }

        if now.elapsed().as_millis() >= 10 {
            if check_for_collision(&paddle, &ball.sprite) {
                if dir == Direction::Up {
                    ball.vel_y += 1;
                }
                if dir == Direction::Down {
                    ball.vel_y -= 1;
                }
                ball.vel_x *= -1;
                ball.vel_y *= -1;
            }
            if check_for_collision(&paddle_enemy, &ball.sprite) {
                if dir_enemy == Direction::Up {
                    ball.vel_y += 1;
                }
                if dir_enemy == Direction::Down {
                    ball.vel_y -= 1;
                }
                ball.vel_x *= -1;
                ball.vel_y *= -1;
            }
            if y_ball < 0 || y_ball > 500
            {
                
                ball.vel_y *= -1;
            }
            if x_ball < 0 || x_ball > 500
            { 
                ball.sprite.set_position((250,250));
                ball.vel_x = ball.vel_x * -1 - 1;
                ball.vel_y = ball.vel_y * -1 - 1;

            }

            ball.sprite.translate((ball.vel_x, ball.vel_y));
        }
        if now.elapsed().as_millis() >= 10 {
            if (y_enemy - CORNER) < 0 {
                dir_enemy = Direction::Still;
                paddle_enemy.set_position((x_enemy, 1 + CORNER));
            } else if (y_player + CORNER) > 500 {
                dir_enemy = Direction::Still;
                paddle_enemy.set_position((x_enemy, 500 - CORNER));
            }

            match dir_enemy {
                Direction::Up => {
                    paddle_enemy.translate((0, 2));
                }
                Direction::Down => {
                    paddle_enemy.translate((0, -2));
                }
                _ => (),
            };
        }
        if now.elapsed().as_millis() >= 10 {
            if (y_player - CORNER) < 0 {
                dir = Direction::Still;
                paddle.set_position((x_player, 1 + CORNER));
            } else if (y_player + CORNER) > 500 {
                dir = Direction::Still;
                paddle.set_position((x_player, 500 - CORNER));
            }
            if held {
                match dir {
                    Direction::Up => {
                        paddle.translate((0, 2));
                    }
                    Direction::Down => {
                        paddle.translate((0, -2));
                    }
                    _ => (),
                };
            } 
            now = Instant::now();
        }
        // FPS COUNTER STUFF
        if now2.elapsed().as_millis() >= 1000
        {
            println!("{counter}");
            counter = 0;
            now2 = Instant::now();
        }
        counter += 1;

        ball.sprite.draw(ctx).unwrap();
        paddle.draw(ctx).unwrap();
        paddle_enemy.draw(ctx).unwrap();
        
    })
    .unwrap();
}
