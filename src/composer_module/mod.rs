pub mod composer {
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use sdl2::render::Canvas;
    use sdl2::render::BlendMode;
    use sdl2::video::Window;


    pub struct LayerComposer{
        layer_buffer: [[u8; 256]; 240],
        target: Canvas<Window>,
    }

    impl LayerComposer{
        pub fn new(mut target: Canvas<Window>) -> Self{
            let mut buffer = [[0; 256]; 240];
            target.set_blend_mode(BlendMode::Blend);
            LayerComposer{layer_buffer: buffer, target: target}
        }

        pub fn render(&mut self, player_coords: (i32, i32), layers: Vec<Vec<(Rect, Color)>>) -> () {
            self.target.clear();
            let (x_size, y_size) = self.target.output_size().unwrap();
            self.target.set_draw_color(Color::RGB(100, 150, 200));
            self.target.fill_rect(Rect::new(0, 0, x_size, y_size)).unwrap();
            for layer in layers.into_iter(){
                for element in layer.into_iter(){
                    self.target.set_draw_color(element.1);
                    self.target.fill_rect(element.0);
                }
            }
            self.target.set_draw_color(Color::RGBA(0, 0, 0, 155));
            self.target.fill_rect(Rect::new(player_coords.0, player_coords.1, 16, 16)).unwrap();
            self.target.present();

        }
    }
}
