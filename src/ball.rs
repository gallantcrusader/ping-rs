use cat_box::Sprite;

pub struct Ball {
    pub sprite: Sprite,
    pub vol_x: i32,
    pub vol_y: i32,
}

impl Ball {
    pub fn new(x: i32, y: i32) -> Ball {
        Ball {
            sprite: Sprite::from_bytes(include_bytes!("../ping.png"), x, y).unwrap(),
            vol_x: -1,
            vol_y: 0,
        }
    }
}
