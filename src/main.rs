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
use sdl2::rect::Rect;


fn main() {
    //Create two_stroke objects
    let mut mb = omnibus::Omnibus::new("bus");
    let count = clock_module::clock::TheCount::new(Duration::new(1, 0), 10, &mut mb);
    let mut bad_rand = rng_module::bad_rng::StatefulLfsr::new(11, 11, &mut mb);
    //Start threads for two_stroke objects
    let thread1 = thread::spawn(move || {count.run();});
    let thread2 = thread::spawn(move || {mb.do_messaging();});
    let thread3 = thread::spawn(move || {bad_rand.run();});
    //Create sdl window to allow for input capture and display
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("two_stroke demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut events = sdl_context.event_pump().unwrap();


    let mut i: u8 = 1;
    let mut j: u8 = 2;
    let mut k: u8 = 3;
    loop{
        i = i.wrapping_add(1);
        j = j.wrapping_add(i);
        k = k.wrapping_add(j);
        canvas.clear();
        canvas.set_draw_color(Color::RGB(i, j, k));
        canvas.fill_rect(Rect::new(10, 10, 780, 580));
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        for event in events.poll_iter(){
            match event{ //Input handling goes here now and send input out to logic
                Event::KeyUp {keycode: Some(Keycode::W), ..} | Event::KeyDown {keycode: Some(Keycode::W), ..} => {
                    println!("Key W: {:?}", event);},
                Event::KeyUp {keycode: Some(Keycode::A), ..} | Event::KeyDown {keycode: Some(Keycode::A), ..} => {
                    println!("KeyUp A: {:?}", event);},
                Event::KeyUp {keycode: Some(Keycode::S), ..} | Event::KeyDown {keycode: Some(Keycode::S), ..} => {
                    println!("KeyUp S: {:?}", event);},
                Event::KeyUp {keycode: Some(Keycode::D), ..} | Event::KeyDown {keycode: Some(Keycode::D), ..} => {
                    println!("KeyUp D: {:?}", event);},
                Event::Quit {..} => {exit(1)},
                _ => {}
            }
        }
        //Call render here
        canvas.present(); //with rendered content. Possibly hand canvas off to the renderer
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
        //Wait on clock tick here
    }


    let mut my_input = input_module::input_scanner::Inputmanager::new(12, &mut mb, events);
    //mb.publish(Arc::new(omnibus::Message::new_sub("bus", 2, "test", Instant::now())));
    my_input.run();

}
