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

    let count = clock_module::clock::TheCount::new(Duration::new(1, 0), 10, &mut mb);
    let mut bad_rand = bad_rng::StatefulLfsr::new(11, 11, &mut mb);
    let my_input = input_module::input_scanner::Inputmanager::new(12, &mut mb);


    mb.publish(Arc::new(omnibus::Message::new_sub("bus", 2, "test")));
    let thread1 = thread::spawn(move || {
        count.run();
    });
    let thread2 = thread::spawn(move || {
        mb.do_messaging();
    });
    let thread2 = thread::spawn(move || {
        bad_rand.run();
    });
    my_input.run();

}
