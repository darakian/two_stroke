pub mod clock {
    use std::time::{Duration, Instant};
    use std::thread;
    use std::sync::Arc;
    extern crate crossbeam_channel;
    use messaging_module::omnibus;
    use messaging_module::omnibus::Message;


    pub struct TheCount{
        start_time: Instant,
        tick_step: Duration,
        message_id: u64,
        sender: crossbeam_channel::Sender<Arc<Message>>,
        reciever: crossbeam_channel::Receiver<Arc<Message>>
    }

    impl TheCount{
        pub fn new(step: Duration, id: u64, channels: (crossbeam_channel::Sender<Arc<Message>>, crossbeam_channel::Receiver<Arc<Message>>)) -> Self{
            TheCount{start_time: Instant::now(), tick_step: step, message_id: id, sender: channels.0, reciever: channels.1}
        }

        pub fn run(&self){
            loop{
                self.sender.send(
                    Arc::new(omnibus::Message::new_tick("all", self.message_id, Instant::now())))
                .unwrap();

                thread::sleep(self.tick_step);
            }
        }
    }
}
