mod messaging_module;
use messaging_module::omnibus;
mod clock_module;
use std::time::Duration;
mod bad_rng;
use bad_rng::lfsr_rng;
use std::sync::Arc;


fn main() {
    let mut bad_rand = lfsr_rng::StatefulLfsr::new(11);
    let mut mb = omnibus::Omnibus::new("bus");
    let count = clock_module::clock::TheCount::new(Duration::new(1, 0), 10, mb.join(10).unwrap());
    mb.publish(Arc::new(omnibus::Message::new_sub("bus", 2, "test")));
    bad_rand.step();
    //println!("Hello, world! {}", bad_rand.sample());
    mb.do_messaging();
}
