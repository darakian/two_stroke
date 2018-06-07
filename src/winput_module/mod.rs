pub mod window_input {
    use std::time::{Duration, Instant};
    use std::thread;
    use std::sync::Arc;
    extern crate crossbeam_channel;
    use messaging_module::omnibus;
    use messaging_module::omnibus::Message;
    use winit::{Event, WindowEvent};

    pub struct winput{
    }

    impl winput{
        pub fn run(){
            loop {
                events_loop.poll_events(|event| {
                    match event {
                    Event::WindowEvent { event: WindowEvent::Resized(w, h), .. } => {
                        println!("The window was resized to {}x{}", w, h);
                        },
                    _ => ()
                    }
                });
            }
        }
    }
}
