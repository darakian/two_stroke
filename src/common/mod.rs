pub mod player_action {

    #[derive(Debug, Clone)]
    pub struct PlayerInput{
        movement: Direction,
        jump: bool,
        shoot: bool,
    }

    impl PlayerInput{
        pub fn new(x_val: u8, y_val: u8, jumping: bool, shooting: bool) -> Self{

        }

    }

    #[derive(Debug, Clone)]
    pub enum Direction{
        Neutral(),
        Up(),
        UpLeft(),
        UpRight(),
        Left(),
        Right(),
        Down(),
        DownLeft(),
        DownRight(),
    }
}
