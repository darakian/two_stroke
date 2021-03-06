mod common;
mod messaging_module;
mod clock_module;
mod rng_module;
mod composer_module;
use messaging_module::omnibus;
use messaging_module::omnibus::OmniPayload;
use std::time::Duration;
use std::thread;
use std::process::exit;
use std::sync::Arc;
use std::time::Instant;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crossbeam_channel::unbounded;

struct background {
    components: Vec<(Rect, Color)>,
}


fn main() {
    //Create two_stroke objects
    let mut message_bus = omnibus::Omnibus::new("general_bus");
    let (main_send, main_recv) = message_bus.join(1).unwrap();
    let mut bad_rand = rng_module::bad_rng::StatefulLfsr::new(11, 11, &mut message_bus);
    let count = clock_module::clock::TheCount::new(Duration::new(0, 16666666), 10, &mut message_bus);
    //Start threads for two_stroke objects
    let _thread1 = thread::spawn(move || {count.run();});
    let _thread2 = thread::spawn(move || {bad_rand.run();});
    //Create sdl window to allow for input capture and display

    let x_size = 800;
    let y_size = 600;
    let sdl_context = sdl2::init().unwrap();
    let window = sdl_context.video().unwrap().window("two_stroke demo", 800, 600)
                .position_centered()
                .resizable()
                .opengl()
                .build()
                .unwrap();

    println!("{:?}", sdl_context.video().unwrap().current_video_driver());
    let mut canvas = window.into_canvas().build().unwrap();
    let mut layer_composer = composer_module::composer::LayerComposer::new(canvas);
    let mut events = sdl_context.event_pump().unwrap();
    let mbus_thread = thread::spawn(move || {message_bus.do_messaging();});
    let mut layer = [[0; 256]; 240];
    let mut player_coords = (0, 0);
    let mut player_velocity = (0, 0);
    let mut current_time = Instant::now();





    //Begin main game loop
    //SDL needs to be on the main thread so video/audio/input are all here.
    let frame_buffer = [[0u8; 256]; 240];

    //Dummy video variables
    let mut i: u8 = 1;
    let mut j: u8 = 2;
    let mut k: u8 = 3;

    let mut x_offset: i32 = 0;
    let mut y_offset: i32 = 0;
    loop{
        main_send.send(
                    Arc::new(omnibus::Message::new_layer("composer", frame_buffer, 0, current_time)))
                .expect("Error sending tick");

        //Check Input and send messages
        for event in events.poll_iter(){
            println!(">>> {:?}", event);
            match event{ //Input handling goes here now and send input out to logic
                Event::KeyDown {keycode: Some(Keycode::W), ..} => {
                    println!("Key W: {:?}", event);
                    player_velocity.1 = -8;
                    println!("Velo: {:?}", player_velocity);
                },
                Event::KeyDown {keycode: Some(Keycode::A), ..} => {
                    println!("Key A: {:?}", event);
                    player_velocity.0 = -8;
                    println!("Velo: {:?}", player_velocity);
                },
                Event::KeyDown {keycode: Some(Keycode::S), ..} => {
                    println!("Key S: {:?}", event);
                    player_velocity.1 = 8;
                    println!("Velo: {:?}", player_velocity);
                },
                Event::KeyDown {keycode: Some(Keycode::D), ..} => {
                    println!("Key D: {:?}", event);
                    player_velocity.0 = 8;
                    println!("Velo: {:?}", player_velocity);
                },
                Event::KeyUp {keycode: Some(Keycode::A), ..} | Event::KeyUp {keycode: Some(Keycode::D), ..} => {
                    println!("Key A: {:?}", event);
                    player_velocity.0 = 0;
                    println!("Velo: {:?}", player_velocity);
                },
                Event::KeyUp {keycode: Some(Keycode::W), ..} | Event::KeyUp {keycode: Some(Keycode::S), ..} => {
                    println!("Key W: {:?}", event);
                    player_velocity.1 = 0;
                    println!("Velo: {:?}", player_velocity);
                },
                Event::Quit {..} => {exit(1)},
                _ => {}
            }
            player_coords.0 += player_velocity.0;
            player_coords.1 += player_velocity.1;
        }
        //Read messages and configure variables as needed

        //Render phase
        layer_composer.render(player_coords, vec![
            vec![
                (Rect::new(200, 100, 50, 50), Color::RGB(200, 150, 200)),
                (Rect::new(200, 200, 50, 50), Color::RGB(200, 150, 200)),
                (Rect::new(200, 300, 50, 50), Color::RGB(200, 150, 200)),
                (Rect::new(200, 500, 50, 50), Color::RGB(200, 150, 200))
                ]]);


        //sweep(&mut canvas);


        // Wait on clock tick here
        for msg in main_recv.iter(){
            match msg.payload{
                Some(OmniPayload::Tick(now)) => {current_time = now; break}, //this breaks the iter loop and allows the outer loop to complete
                _ => {},
            }
        }
    }


    //let mut my_input = input_module::input_scanner::Inputmanager::new(12, &mut message_bus, events);
    //message_bus.publish(Arc::new(omnibus::Message::new_sub("bus", 2, "test", Instant::now())));
    //my_input.run();

}

fn sweep(mut canvas: &mut Canvas<Window>) -> (){
    for i in (-800..=800).step_by(1){
        canvas.clear();
        canvas.fill_rect(Rect::new(i, 0, 800, 600)).unwrap();
    }
}
