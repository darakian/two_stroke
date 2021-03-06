pub mod bad_rng{
	use std::sync::Arc;
    extern crate crossbeam_channel;
    use crate::messaging_module::omnibus;
	use crate::messaging_module::omnibus::{Message, OmniPayload, Omnibus};
	use std::time::{Instant};


	pub struct StatefulLfsr{
		state: u16,
		message_id: u64,
        sender: crossbeam_channel::Sender<Arc<Message>>,
        reciever: crossbeam_channel::Receiver<Arc<Message>>,
		current_tick: Instant,
	}

	impl  StatefulLfsr{

		pub fn new(seed: u16, id: u64, message_bus: &mut Omnibus) -> StatefulLfsr{
		let channels = message_bus.join(id).unwrap();
		StatefulLfsr{state: seed, message_id: id, sender: channels.0, reciever: channels.1, current_tick: Instant::now()}
		}

		pub fn step(&mut self){
		let tmp: u16 = ((self.state >> 0) ^ (self.state >> 2) ^ (self.state >> 3) ^ (self.state >> 5) ) & 1;
		self.state = (self.state >> 1) | (tmp << 15);
		}

		pub fn sample(&mut self) -> u16{
		let current = self.state;
		self.step();
		return current;
		}

		pub fn run(&mut self) {
			loop{
				let msg = self.reciever.recv().unwrap();
				match msg.payload{
                    Some(ref kind) => {
                    match kind {
                        OmniPayload::Quit => return,
                        OmniPayload::RngRequest(count) => {
							for _i in 0..*count {
								let rng_value = self.sample();
								self.sender.send(
									Arc::new(omnibus::Message::new_rng("rng", self.message_id, rng_value, self.current_tick)))
								.unwrap();
							}
						},
						OmniPayload::Tick(new_time) => {self.current_tick = *new_time},
                        _ => {}
                        }
                    },
                    None => {}
                }
			}
		}
	}
}
