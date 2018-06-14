pub mod input_scanner {
    extern crate sdl2;
    use self::sdl2::keyboard::Keycode;
    use self::sdl2::keyboard::Scancode;
    use std::collections::HashSet;

    pub struct Inputmanager{
        last_key_state: HashSet<Scancode>,
        sld_context: sdl2::Sdl,
        event_pump: sdl2::EventPump
    }

    impl Inputmanager{
        pub fn new() -> Inputmanager{
            let context = sdl2::init().unwrap();
            let events = context.event_pump().unwrap();
            Inputmanager{last_key_state: HashSet::new(), sld_context: context, event_pump: events}
        }

        fn pressed_scancode_set(e: &sdl2::EventPump) -> HashSet<Scancode> {
            e.keyboard_state().pressed_scancodes().collect()
        }

        fn pressed_keycode_set(e: &sdl2::EventPump) -> HashSet<Keycode> {
            e.keyboard_state().pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect()
        }

        fn newly_pressed(old: &HashSet<Scancode>, new: &HashSet<Scancode>) -> HashSet<Scancode> {
            new - old
            // sugar for: new.difference(old).collect()
        }

    }

}
