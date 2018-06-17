pub mod bad_rng{
	use std::sync::Arc;
    extern crate crossbeam_channel;
    use messaging_module::omnibus;
	use messaging_module::omnibus::{Message, OmniPayload, Omnibus};

	pub struct StatefulLfsr{
		state: u16,
		message_id: u64,
        sender: crossbeam_channel::Sender<Arc<Message>>,
        reciever: crossbeam_channel::Receiver<Arc<Message>>
	}

	impl StatefulLfsr{

		pub fn new(seed: u16, id: u64, message_bus: &mut Omnibus) -> StatefulLfsr{
		let channels = message_bus.join(id).unwrap();
		StatefulLfsr{state: seed, message_id: id, sender: channels.0, reciever: channels.1}
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
	}
}
