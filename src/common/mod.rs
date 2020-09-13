pub mod player_action {

    #[derive(Debug, Clone)]
    pub struct PlayerInput{
        xmovement: Direction,
        ymovement: Direction,
        jump: bool,
        shoot: bool,
    }

    impl PlayerInput{
        pub fn new(x_val: i16, y_val: i16, jumping: bool, shooting: bool) -> Self{
            PlayerInput{xmovement: Direction::XDirection{x: x_val}, ymovement: Direction::YDirection{y: y_val}, jump: jumping, shoot: shooting}
        }
    }

    #[derive(Debug, Clone)]
    pub enum Direction{
        XDirection{x: i16},
        YDirection{y: i16},
    }
}

use std::fs::File;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::io::Read;

pub fn bmp_to_rects(mut f: File) -> Vec<(Rect, Color)>{
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    vec![]
}
