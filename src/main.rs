mod messaging_module;
use messaging_module::omnibus;
mod clock_module;
use std::time::Duration;
mod rng_module;
use rng_module::bad_rng;
use std::sync::Arc;
use std::thread;
mod input_module;
use input_module::input_scanner;



fn main() {
    let mut mb = omnibus::Omnibus::new("bus");

    let count = clock_module::clock::TheCount::new(Duration::new(1, 0), 10, mb.join(10).unwrap());
    let mut bad_rand = bad_rng::StatefulLfsr::new(11, 11, mb.join(10).unwrap());

    mb.publish(Arc::new(omnibus::Message::new_sub("bus", 2, "test")));
    bad_rand.step();
    let h1 = thread::spawn(move || {
        count.run();
    });
    let my_input = input_module::input_scanner::Inputmanager::new();
    //println!("Hello, world! {}", bad_rand.sample());
    mb.do_messaging();
    loop {
        my_input.print_scancodes();
    }
}
