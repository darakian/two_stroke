mod common;
mod messaging_module;
mod clock_module;
mod input_module;
mod rng_module;
use messaging_module::omnibus;
use std::time::Duration;
use std::sync::Arc;
use std::thread;
use std::process::exit;

extern crate sdl2;
use self::sdl2::keyboard::Scancode;
use std::collections::HashSet;
use sdl2::event::Event;
use sdl2::event::EventType;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut events = sdl_context.event_pump().unwrap();

    //Create window to get keyboard input


    loop{
        for event in events.poll_iter(){
            println!("Event = {:?}", event);
            match event{
                Event::KeyDown {..} => {println!("Keyevent = {:?}", event);},
                Event::Quit {..} => {exit(1)},
                _ => {println!("Unknown Event == {:?}", event);}
            }
        }
        //canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    let mut mb = omnibus::Omnibus::new("bus");

    let count = clock_module::clock::TheCount::new(Duration::new(1, 0), 10, &mut mb);
    let mut bad_rand = rng_module::bad_rng::StatefulLfsr::new(11, 11, &mut mb);
    let mut my_input = input_module::input_scanner::Inputmanager::new(12, &mut mb, events);


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
