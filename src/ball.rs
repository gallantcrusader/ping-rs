use cat_box::Sprite;
use rand::{self};

pub struct Ball {
    pub sprite: Sprite,
    pub vel_x: i32,
    pub vel_y: i32,
}

impl Ball {
    pub fn new(x: i32, y: i32) -> Ball {
        let xbool = rand::random();
        let ybool = rand::random();
        let x_a: i32;
        let y_a: i32;
        if xbool {
            x_a = 1;
        } else {
            x_a = -1;
        }
        if ybool {
            y_a = 1;
        } else {
            y_a = -1;
        }

        Ball {
            sprite: Sprite::from_bytes(include_bytes!("../ping.png"), x, y).unwrap(),
            vel_x: x_a,
            vel_y: y_a,
        }
    }

    pub fn rand_angle(&mut self) {
        let xbool = rand::random();
        let ybool = rand::random();
        let x_a: i32;
        let y_a: i32;
        if xbool {
            x_a = 1;
        } else {
            x_a = -1;
        }
        if ybool {
            y_a = 1;
        } else {
            y_a = -1;
        }

        self.vel_x = x_a;
        self.vel_y = y_a;
    }
}
