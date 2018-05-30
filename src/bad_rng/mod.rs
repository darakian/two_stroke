// fn main(){
// 	//println!("Hello rust");
// 	let mut my_random = k_random::WikiLsfr::new(4);
// 	for i in 1..100{
// 	println!("{}", my_random.sample());
// 	}
// }

pub mod lfsr_rng{

	pub struct stateful_lfsr{
		state: u16,
	}

	impl stateful_lfsr{

		pub fn new(seed: u16) -> stateful_lfsr{
		stateful_lfsr{state: seed}
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
