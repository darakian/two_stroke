mod common;
mod messaging_module;
mod clock_module;
mod rng_module;
mod composer_module;
use messaging_module::omnibus;
use messaging_module::omnibus::{Message, OmniPayload, Omnibus};
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
use sdl2::rect::Rect;


fn main() {
    //Create two_stroke objects
    let mut message_bus = omnibus::Omnibus::new("bus");
    let (main_send, main_recv) = message_bus.join(1).unwrap();
    let mut bad_rand = rng_module::bad_rng::StatefulLfsr::new(11, 11, &mut message_bus);
    let mut layer_composer = composer_module::composer::layer_composer::new(13, &mut message_bus);
    let count = clock_module::clock::TheCount::new(Duration::new(0, 16666666), 10, &mut message_bus);
    //Start threads for two_stroke objects
    let _thread1 = thread::spawn(move || {count.run();});
    let _thread2 = thread::spawn(move || {bad_rand.run();});
    let _thread3 = thread::spawn(move || {layer_composer.run();});
    //Create sdl window to allow for input capture and display

    let sdl_context = sdl2::init().unwrap();
    let window = sdl_context.video().unwrap().window("two_stroke demo", 800, 600)
                .position_centered()
                .build()
                .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = sdl_context.event_pump().unwrap();
    let mbus_thread = thread::spawn(move || {message_bus.do_messaging();});





    //Begin main game loop
    //SDL needs to be on the main thread so video/audio/input are all here.

    //Dummy video variables
    let mut i: u8 = 1;
    let mut j: u8 = 2;
    let mut k: u8 = 3;
    loop{
        //Check Input and send messages
        for event in events.poll_iter(){
            match event{ //Input handling goes here now and send input out to logic
                Event::KeyUp {keycode: Some(Keycode::W), ..} | Event::KeyDown {keycode: Some(Keycode::W), ..} => {
                    println!("Key W: {:?}", event);},
                Event::KeyUp {keycode: Some(Keycode::A), ..} | Event::KeyDown {keycode: Some(Keycode::A), ..} => {
                    println!("Key A: {:?}", event);},
                Event::KeyUp {keycode: Some(Keycode::S), ..} | Event::KeyDown {keycode: Some(Keycode::S), ..} => {
                    println!("Key S: {:?}", event);},
                Event::KeyUp {keycode: Some(Keycode::D), ..} | Event::KeyDown {keycode: Some(Keycode::D), ..} => {
                    println!("Key D: {:?}", event);},
                Event::Quit {..} => {exit(1)},
                _ => {}
            }
        }
        //Read messages and configure variables as needed

        //Render phase
        i = i.wrapping_add(1);
        j = j.wrapping_add(2);
        k = k.wrapping_add(3);
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.set_draw_color(Color::RGB(i, j, k));
        canvas.fill_rect(Rect::new(0, 0, 800, 600)).unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.present();



        //Wait on clock tick here
        for msg in main_recv.iter(){
            match msg.payload{
                Some(OmniPayload::Tick(now)) => {break}, //this breaks the iter loop and allows the outer loop to complete
                _ => {},
            }
        }
    }


    //let mut my_input = input_module::input_scanner::Inputmanager::new(12, &mut message_bus, events);
    //message_bus.publish(Arc::new(omnibus::Message::new_sub("bus", 2, "test", Instant::now())));
    //my_input.run();

}
