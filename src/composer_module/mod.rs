pub mod composer {
    use std::time::Instant;
    use std::sync::Arc;
    use crossbeam_channel::{Sender, Receiver};
    use crate::messaging_module::omnibus;
    use crate::messaging_module::omnibus::{Message, OmniPayload, Omnibus};
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use sdl2::render::Canvas;
    use sdl2::video::Window;


    pub struct LayerComposer{
        layer_buffer: [[u8; 256]; 240],
        target: Canvas<Window>,
    }

    impl LayerComposer{
        pub fn new(target: Canvas<Window>) -> Self{
            let mut buffer = [[0; 256]; 240];
            LayerComposer{layer_buffer: buffer, target: target}
        }

        pub fn render(&mut self, player_coords: (i32, i32)) -> () {
            self.target.clear();
            let (x_size, y_size) = self.target.output_size().unwrap();
            self.target.set_draw_color(Color::RGB(100, 150, 200));
            self.target.fill_rect(Rect::new(0, 0, x_size, y_size)).unwrap();
            self.target.set_draw_color(Color::RGB(0, 0, 0));
            self.target.fill_rect(Rect::new(player_coords.0, player_coords.1, 16, 16)).unwrap();
            self.target.present();

        }
    }
}
