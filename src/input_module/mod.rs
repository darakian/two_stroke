pub mod input_scanner {
    extern crate sdl2;
    use self::sdl2::keyboard::Keycode;
    use self::sdl2::keyboard::Scancode;
    use std::collections::HashSet;
    use std::sync::Arc;
    extern crate crossbeam_channel;
    use messaging_module::omnibus;
    use messaging_module::omnibus::{Message, OmniPayload};

    pub struct Inputmanager{
        last_key_state: HashSet<Scancode>,
        sld_context: sdl2::Sdl,
        event_pump: sdl2::EventPump,
        message_id: u64,
        sender: crossbeam_channel::Sender<Arc<Message>>,
        reciever: crossbeam_channel::Receiver<Arc<Message>>
    }

    impl Inputmanager{
        pub fn new(id: u64, channels: (crossbeam_channel::Sender<Arc<Message>>, crossbeam_channel::Receiver<Arc<Message>>)) -> Inputmanager{
            let context = sdl2::init().unwrap();
            let events = context.event_pump().unwrap();
            Inputmanager{last_key_state: HashSet::new(), sld_context: context, event_pump: events, message_id: id, sender: channels.0, reciever: channels.1}
        }

        fn pressed_scancode_set(&self) -> HashSet<Scancode> {
            self.event_pump.keyboard_state().pressed_scancodes().collect()
        }

        fn pressed_keycode_set(&self) -> HashSet<Keycode> {
            self.event_pump.keyboard_state().pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect()
        }

        fn newly_pressed(old: &HashSet<Scancode>, new: &HashSet<Scancode>) -> HashSet<Scancode> {
            new - old
        }

        fn print_scancodes(&self){
            for code in Inputmanager::pressed_keycode_set(self){
                println!("{:?}", code);
            }
        }

        pub fn run(&self){
            loop{
                let msg = self.reciever.recv().unwrap();

                match msg.payload{
                    Some(ref kind) => {
                    match kind {
                        OmniPayload::Quit => return,
                        OmniPayload::Tick(now) => {
                            self.print_scancodes();
                            println!("Sending input now: {:?}", now);
                            }
                        _ => {}
                        }
                    },
                    None => {}
                }


            }
        }

    }

}
