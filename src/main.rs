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


fn main() {
    //Create two_stroke objects
    let mut message_bus = omnibus::Omnibus::new("general_bus");
    let (render_send, render_recv) = unbounded::<[[u8; 256]; 240]>();
    let (main_send, main_recv) = message_bus.join(1).unwrap();
    let mut bad_rand = rng_module::bad_rng::StatefulLfsr::new(11, 11, &mut message_bus);
    let mut layer_composer = composer_module::composer::LayerComposer::new();
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
    let mut events = sdl_context.event_pump().unwrap();
    let mbus_thread = thread::spawn(move || {message_bus.do_messaging();});
    let mut layer = [[0; 256]; 240];
    let mut player_coords = (0, 0);
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
                Event::KeyUp {keycode: Some(Keycode::W), ..} | Event::KeyDown {keycode: Some(Keycode::W), ..} => {
                    println!("Key W: {:?}", event);
                    player_coords.1 = (player_coords.1 - 3)%800;
                    println!("coords: {:?}", player_coords);
                },
                Event::KeyUp {keycode: Some(Keycode::A), ..} | Event::KeyDown {keycode: Some(Keycode::A), ..} => {
                    println!("Key A: {:?}", event);
                    player_coords.0 = (player_coords.0 - 3)%600;
                    println!("coords: {:?}", player_coords);
                },
                Event::KeyUp {keycode: Some(Keycode::S), ..} | Event::KeyDown {keycode: Some(Keycode::S), ..} => {
                    println!("Key S: {:?}", event);
                    player_coords.1 = (player_coords.1 + 3)%800;
                    println!("coords: {:?}", player_coords);
                },
                Event::KeyUp {keycode: Some(Keycode::D), ..} | Event::KeyDown {keycode: Some(Keycode::D), ..} => {
                    println!("Key D: {:?}", event);
                    player_coords.0 = (player_coords.0 + 3)%600;
                    println!("coords: {:?}", player_coords);
                },
                Event::Quit {..} => {exit(1)},
                _ => {}
            }
        }
        //Read messages and configure variables as needed

        //Render phase
        render_send.send(layer);


        i = i.wrapping_add(1);
        j = j.wrapping_add(2);
        k = k.wrapping_add(3);

        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.set_draw_color(Color::RGB(i, j, k));
        canvas.fill_rect(Rect::new(0+x_offset, 0+y_offset, x_size, y_size)).unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(player_coords.0, player_coords.1, 16, 16)).unwrap();
        canvas.present();
        //sweep(&mut canvas);


        let layer = render_recv.recv();
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
