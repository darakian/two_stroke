mod messaging_module;
use messaging_module::omnibus;
mod clock;
mod bad_rng;
use bad_rng::lfsr_rng;

fn main() {
    let mut badRand = lfsr_rng::stateful_lfsr::new(11);
    let mut mb = omnibus::Omnibus::new("bus");
    mb.publish(omnibus::Message::new_sub("bus", 2, "test"));
    badRand.step();
    //println!("Hello, world! {}", badRand.sample());
    mb.do_messaging();
}
