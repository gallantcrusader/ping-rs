use cat_box::{
    draw_text, get_keyboard_state, physics::check_for_collision, Game, Sprite, TextMode,
};
use sdl2::keyboard::Scancode;
//use sdl2::event::Event;
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

    let mut dir = Direction::Still;
    let mut dir_enemy = Direction::Still;

    let mode = TextMode::Transparent {
        colour: (255, 255, 255),
    };
    let mut score_player = 0;
    let mut score_enemy = 0;

    let mut now = Instant::now();

    game.run(|ctx| {
        ctx.set_background_colour(0, 0, 0);

        let (x_player, y_player) = paddle.position().into();
        let (x_enemy, y_enemy) = paddle_enemy.position().into();

        let (x_ball, y_ball) = ball.sprite.position().into();

        if now.elapsed().as_millis() >= 1 {
            let keys = get_keyboard_state(ctx).keys;
            for key in &keys {
                match key {
                    Scancode::W | Scancode::Up => {
                        dir = Direction::Up;
                    }
                    Scancode::S | Scancode::Down => {
                        dir = Direction::Down;
                    }
                    Scancode::Q | Scancode::Escape => {
                        game.terminate();
                    }
                    _ => (),
                };
            }
        }

        if y_enemy < y_ball {
            dir_enemy = Direction::Down;
        }
        if y_enemy > y_ball {
            dir_enemy = Direction::Up;
        }

        if now.elapsed().as_millis() >= 10 {
            //BALL
            if check_for_collision(&paddle, &ball.sprite) 
                && x_ball == 50 
                {
                if ball.vel_y < i32::MAX - 1 {
                    ball.vel_y += ball.vel_y * 1;
                    ball.vel_x *= -1;
                    ball.vel_y *= -1;
                }
            }
            if check_for_collision(&paddle_enemy, &ball.sprite) && (x_ball >= 449 && x_ball <= 469)
            {
                if ball.vel_y < i32::MAX - 1 {
                    ball.vel_y += ball.vel_y * 1;
                    ball.vel_x *= -1;
                    ball.vel_y *= -1;
                }
            }
            if y_ball < 0 || y_ball > 500 {
                ball.vel_y *= -1;
            }
            if x_ball < 0 {
                ball.sprite.set_position((250, 250));
                ball::Ball::rand_angle(&mut ball);
                score_enemy += 1;
            } else if x_ball > 500 {
                ball.sprite.set_position((250, 250));
                ball::Ball::rand_angle(&mut ball);
                score_player += 1;
            }

            ball.sprite.translate((ball.vel_x, ball.vel_y));

            //ENEMY LOGIC
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
            dir_enemy = Direction::Still;

            //PLAYEW LOGIC
            if (y_player - CORNER) < 0 {
                dir = Direction::Still;
                paddle.set_position((x_player, 1 + CORNER));
            } else if (y_player + CORNER) > 500 {
                dir = Direction::Still;
                paddle.set_position((x_player, 500 - CORNER));
            }

            match dir {
                Direction::Up => {
                    paddle.translate((0, 3));
                }
                Direction::Down => {
                    paddle.translate((0, -3));
                }
                _ => (),
            };
            dir = Direction::Still;

            now = Instant::now();
        }

        draw_text(
            ctx,
            format!("{score_player}"),
            "fira.ttf",
            50,
            (90, 100),
            mode,
        )
        .unwrap();
        draw_text(
            ctx,
            format!("{score_enemy}"),
            "fira.ttf",
            50,
            (500 - 90, 100),
            mode,
        )
        .unwrap();

        ball.sprite.draw(ctx).unwrap();
        paddle.draw(ctx).unwrap();
        paddle_enemy.draw(ctx).unwrap();
    })
    .unwrap();
}

/* fn is_pressed(e: &sdl2::EventPump, k: Scancode) -> bool {
    e.keyboard_state().is_scancode_pressed(k)
} */
