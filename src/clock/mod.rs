// use std::thread;
// use std::sync::mpsc::channel;
//
// fn main() {
//     let start_time = Instant::now();
//     let (tx, rx) = channel();
//     // Threaded one millisecond timer
//     thread::spawn(move || {
//     loop {
//         thread::sleep(Duration::new(0, 16666666));
//         tx.send("tick").unwrap();
//     }
//     });
//
//
//
//
//     for entry in rx.iter() {
//         if start_time.elapsed() >= Duration::from_secs(1) {break}
//     }
//
// }

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
