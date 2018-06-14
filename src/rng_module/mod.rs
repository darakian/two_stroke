pub mod bad_rng{

	pub struct StatefulLfsr{
		state: u16,
	}

	impl StatefulLfsr{

		pub fn new(seed: u16) -> StatefulLfsr{
		StatefulLfsr{state: seed}
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
