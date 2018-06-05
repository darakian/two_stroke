pub mod clock {
    use std::time::{Duration, Instant};
    use std::thread;
    extern crate crossbeam_channel;
    use self::crossbeam_channel::unbounded;
    use messaging_module::omnibus;
    use messaging_module::omnibus::Message;


    pub struct the_count{
        start_time: Instant,
        tick_step: Duration,
        message_id: u64,
        sender: crossbeam_channel::Sender<Message>
    }

    impl the_count{
        pub fn new(step: Duration, id: u64, channel: crossbeam_channel::Sender<Message>) -> Self{
            the_count{start_time: Instant::now(), tick_step: step, message_id: id,sender: channel}
        }

        pub fn run(&self){
            loop{
                self.sender.send(
                    omnibus::Message::new_tick("all", self.message_id, Instant::now()))
                .unwrap();

                thread::sleep(self.tick_step);
            }
        }
    }
}
