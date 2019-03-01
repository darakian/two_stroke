pub mod composer {
    use std::time::{Duration, Instant};
    use std::thread;
    use std::sync::Arc;
    use sdl2::rect::Point;
    extern crate crossbeam_channel;
    use messaging_module::omnibus;
    use messaging_module::omnibus::{Message, OmniPayload, Omnibus};


    pub struct layer_composer{
        layer_buffer: [[u8; 256]; 240],
        message_id: u64,
        sender: crossbeam_channel::Sender<Arc<Message>>,
        reciever: crossbeam_channel::Receiver<Arc<Message>>
    }

    impl layer_composer{
        pub fn new(id: u64, message_bus: &mut Omnibus) -> Self{
            let channels = message_bus.join(id).expect("Unable to join channel");
            let mut buffer = [[0; 256]; 240];
            layer_composer{layer_buffer: buffer, message_id: id, sender: channels.0, reciever: channels.1}
        }

        pub fn run(&mut self){
            loop{
                for row in self.layer_buffer.iter(){
                    for pixel in row.iter(){
                        pixel.wrapping_add(1);
                    }
                }    
            }
        }
    }
}
