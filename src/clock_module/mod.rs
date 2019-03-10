pub mod clock {
    use std::time::{Duration, Instant};
    use std::thread;
    use std::sync::Arc;
    extern crate crossbeam_channel;
    use messaging_module::omnibus;
    use messaging_module::omnibus::{Message, Omnibus};


    pub struct TheCount{
        start_time: Instant,
        tick_step: Duration,
        message_id: u64,
        sender: crossbeam_channel::Sender<Arc<Message>>,
        reciever: crossbeam_channel::Receiver<Arc<Message>>
    }

    impl  TheCount{
        pub fn new(step: Duration, id: u64, message_bus: &mut Omnibus) -> Self{
            let channels = message_bus.join(id).unwrap();
            TheCount{start_time: Instant::now(), tick_step: step, message_id: id, sender: channels.0, reciever: channels.1}
        }

        pub fn run(&self){
            loop{
                self.sender.send(
                    Arc::new(omnibus::Message::new_tick("all", self.message_id, Instant::now())))
                .expect("Error sending tick");

                thread::sleep(self.tick_step);
            }
        }
    }
}
