pub mod composer {
    use std::time::{Duration, Instant};
    use std::thread;
    use std::sync::Arc;
    extern crate crossbeam_channel;
    use messaging_module::omnibus;
    use messaging_module::omnibus::{Message, OmniPayload, Omnibus};


    pub struct layer_composer<'layer_msg: >{
        layer_buffer: [[u8; 256]; 240],
        message_id: u64,
        sender: crossbeam_channel::Sender<Arc<Message<'layer_msg>>>,
        reciever: crossbeam_channel::Receiver<Arc<Message<'layer_msg>>>,
        current_tick: Instant,
    }

    impl <'layer_msg: > layer_composer<'layer_msg>{
        pub fn new(id: u64, message_bus: &'layer_msg mut Omnibus) -> Self{
            let channels = message_bus.join(id).expect("Unable to join channel");
            let mut buffer = [[0; 256]; 240];
            layer_composer{layer_buffer: buffer, message_id: id, sender: channels.0, reciever: channels.1, current_tick: Instant::now()}
        }

        pub fn run(&mut self){
            loop{
                for row in self.layer_buffer.iter(){
                    for pixel in row.iter(){
                        pixel.wrapping_add(1);
                    }
                }
                let send_buff = self.layer_buffer.clone();
                self.sender.send(
                    Arc::new(omnibus::Message::new_layer(&send_buff, self.message_id,  self.current_tick)))
                .expect("Error sending layer reference");

                for msg in self.reciever.iter(){
                    match msg.payload{
                        Some(OmniPayload::Tick(now)) => {self.current_tick = now; break}, //this breaks the iter loop and allows the outer loop to complete
                        _ => {},
                    }
                }
            }
        }
    }
}
