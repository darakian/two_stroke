pub mod composer {
    use std::time::{Duration, Instant};
    use std::thread;
    use std::sync::Arc;
    use sdl2::render::TextureCreator;
    use sdl2::video::WindowContext;
    extern crate crossbeam_channel;
    use messaging_module::omnibus;
    use messaging_module::omnibus::{Message, OmniPayload, Omnibus};


    pub struct texture_composer{
        creator: TextureCreator<WindowContext>,
        message_id: u64,
        sender: crossbeam_channel::Sender<Arc<Message>>,
        reciever: crossbeam_channel::Receiver<Arc<Message>>
    }

    impl texture_composer{
        pub fn new(texture_creator: TextureCreator<WindowContext>, id: u64, message_bus: &mut Omnibus) -> Self{
            let channels = message_bus.join(id).unwrap();
            texture_composer{creator: texture_creator, message_id: id, sender: channels.0, reciever: channels.1}
        }

        pub fn run(&self){
        }
    }
}
