pub mod composer {
    use std::time::Instant;
    use std::sync::Arc;
    use crossbeam_channel::{Sender, Receiver};
    use crate::messaging_module::omnibus;
    use crate::messaging_module::omnibus::{Message, OmniPayload, Omnibus};


    pub struct LayerComposer{
        layer_buffer: [[u8; 256]; 240],
    }

    impl LayerComposer{
        pub fn new() -> Self{
            let mut buffer = [[0; 256]; 240];
            LayerComposer{layer_buffer: buffer}
        }


    }
}
