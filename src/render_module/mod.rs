pub mod render{
    use std::thread;
    use std::sync::Arc;
    extern crate crossbeam_channel;
    use messaging_module::omnibus;
    use messaging_module::omnibus::{Message, OmniPayload, Omnibus};
    extern crate sdl2;
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use sdl2::render::Canvas;
    use sdl2::video::Window;

    pub struct PixelPusher{
        canvas: Canvas<Window>,
        message_id: u64,
        sender: crossbeam_channel::Sender<Arc<Message>>,
        reciever: crossbeam_channel::Receiver<Arc<Message>>
    }

    impl PixelPusher{
        pub fn new(canvas: Canvas<Window>, id: u64, message_bus: &mut Omnibus) -> Self{
            let channels = message_bus.join(id).unwrap();
            PixelPusher{canvas: canvas, message_id: id, sender: channels.0, reciever: channels.1}
        }

        pub fn run(&self){
            // Collect messages for new objects and for objects which are to be added/removed
            // Load/Drop objects as needed from each layer
            // From back layer forward paint canvas
            // Render
            // Wait for next tick
        }
    }
}
