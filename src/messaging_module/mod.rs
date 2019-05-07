#[cfg(test)]
mod tests {
    use messaging_module::{Message, Omnibus};
    use std::thread;

}



pub mod omnibus {
extern crate crossbeam_channel;
use std::sync::Arc;
use std::fmt;
use self::crossbeam_channel::unbounded;
use std::collections::hash_map::{HashMap, Entry};
use std::time::Instant;
use common::player_action::PlayerInput;

#[derive(Debug, Clone)]
pub struct Message{
    publish_tag: String,
    publisher: u64,
    send_timestamp: Instant,
    pub payload: Option<OmniPayload>
}

#[derive(Clone)]
 pub enum OmniPayload {
    Quit,
    Subscribe(String),
    Tick(Instant),
    Input(PlayerInput),
    RngRequest(u8),
    Rng(u16),
    Layer([[u8; 256]; 240]),
    // Move {publish_tag: String, object_tag: String, x: i32, y: i32 },
    // RNG {publish_tag: String, value: u16},
    // Write {publish_tag: String, Message: String},
    // ChangeColor {publish_tag: String, object_tag: String, r: i32, g: i32, b: i32},
    // Sprite {publish_tag: String, object_tag: String,x: i32, y: i32, pixels: Vec<u8>}
}

impl fmt::Debug for OmniPayload{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Omnipaylod: {:?}", self)
    }
}

    impl Message{
        pub fn new_sub(to: &str, from: u64, subscribe_string: &str, timestamp: Instant) -> Self{
            Message{publish_tag: to.to_string(), publisher: from, send_timestamp: timestamp, payload: Some(OmniPayload::Subscribe(subscribe_string.to_string()))}
        }

        pub fn new_tick(to: &str, from: u64, tick_time: Instant) -> Self{
            Message{publish_tag: to.to_string(), publisher: from, send_timestamp: tick_time, payload: Some(OmniPayload::Tick(tick_time))}
        }

        pub fn new_input(to: &str, from: u64, keys: PlayerInput, timestamp: Instant) -> Self{
            Message{publish_tag: to.to_string(), publisher: from, send_timestamp: timestamp, payload: Some(OmniPayload::Input(keys))}
        }

        pub fn new_rng_request(from: u64, count: u8, timestamp: Instant) -> Self{
            Message{publish_tag: "rng".to_string(), publisher: from, send_timestamp: timestamp, payload: Some(OmniPayload::RngRequest(count))}
        }

        pub fn new_rng(to: &str, from: u64, rng_value: u16, timestamp: Instant) -> Self{
            Message{publish_tag: to.to_string(), publisher: from, send_timestamp: timestamp, payload: Some(OmniPayload::Rng(rng_value))}
        }

        pub fn new_layer(to: &str, buffer: [[u8; 256]; 240], from: u64, timestamp: Instant) -> Self{
            Message{publish_tag: to.to_string(), publisher: from, send_timestamp: timestamp, payload: Some(OmniPayload::Layer(buffer))}
        }
    }

    pub struct Omnibus{
        current_tick: Instant,
        bus_id: String,
        global_recv: crossbeam_channel::Receiver<Arc<Message>>,
        global_send: crossbeam_channel::Sender<Arc<Message>>,
        subscribers:  HashMap<u64, (crossbeam_channel::Sender<Arc<Message>>)>,
        feeds: HashMap<String, Vec<crossbeam_channel::Sender<Arc<Message>>>>
    }

    impl Omnibus{
        pub fn new(bus_id: &str) -> Self{
            let (send, receive) = unbounded::<Arc<Message>>();
            let mut bus = Omnibus{current_tick: Instant::now(), bus_id: bus_id.to_string(), global_recv: receive, global_send: send, subscribers: HashMap::new(), feeds: HashMap::new()};
            let (bus_self_tx, bus_self_rx) = bus.join(0).expect("Unable to bind omnibus");
            bus
        }

        pub fn join(&mut self, component_id: u64) -> Result<(crossbeam_channel::Sender<Arc<Message>>, crossbeam_channel::Receiver<Arc<Message>>), &str>{
            let (send, receive) = unbounded::<Arc<Message>>();
            // println!("Attempting to join component {:?}", component_id);
            match self.subscribers.entry(component_id) {
                Entry::Vacant(es) => {
                    es.insert(send.clone());
                    },
                Entry::Occupied(err) => {return Err("Sub_ID in use");}
                }
            self.feeds.entry("all".to_string())
            .and_modify(|vec| {vec.push(send.clone())})
            .or_insert(Vec::new());
            Ok((self.global_send.clone(), receive))
        }

        fn subscribe(&mut self, sub_tag: &str, component_id: u64) -> Result<(), &str>{
            //println!("Adding {:?} to tag {}", component_id, sub_tag);
            let relevant_channel = match self.subscribers.get(&component_id){
                Some(channel) => channel.clone(),
                None => return Err("No such channel")
            };
            self.feeds.entry(sub_tag.to_string())
            .and_modify(|channel_vec| {
                if channel_vec.contains(&relevant_channel) {/*Should handle the case where the channel is already in the channel_vec. TODO*/}
                else {channel_vec.push(relevant_channel.clone());} //Else add the channel to the channel_vec
             })
             //If the vec doesn't exist then make one with the channel
            .or_insert(vec![relevant_channel]);
            Ok(())
        }

        pub fn do_messaging(&mut self) {
            loop {
                let msg = self.global_recv.recv().expect("Error receving message in do_messaging");
                // println!("{:?}", msg);
                // println!("meg_publish tag: {:?}  bus tag: {:?}  same? {:?}", msg.publish_tag, self.bus_id, msg.publish_tag==self.bus_id);
                if self.subscribers.get(&msg.publisher)==None {/*REMOVED FOR TESTING drop(msg); continue;*/}
                if msg.publish_tag == self.bus_id{
                    let pub_tag = msg.publish_tag.clone();
                    let pub_er = msg.publisher;
                    match msg.payload{
                        Some(ref kind) => {
                        //println!("HERE {:?}", kind);
                        match kind {
                            OmniPayload::Quit => return,
                            OmniPayload::Subscribe(sub_tag) => {
                                match self.subscribe(&sub_tag, pub_er){
                                    Ok(value) => {/*Subscription passed. Send message back to subscriber*/},
                                    Err(e) => {/*Subscription failed. Send message back to subscriber*/}
                                };
                                //println!("Also here with {}", sub_tag);
                                }
                            _ => {}
                            }
                        },
                        None => {}
                    }
                } else {
                    match self.feeds.get(&msg.publish_tag){
                        Some(feed_subscribers) => {
                            feed_subscribers.iter().for_each(|x| {
                                // println!("Sending {:?} to {:?}", msg, x);
                            match x.send(Arc::clone(&msg)){
                                Ok(_v) => {},
                                Err(e) => println!("Error sending message {:?}", e),
                                }
                            })
                        },
                        None => {drop(msg)}
                    }
                }
            }
        }


        //Testing methods
        pub fn publish(&mut self, m: Arc<Message>) -> (){
            self.global_send.send(m).unwrap();
        }

        pub fn check_channel_len(&self) -> usize{
            self.global_recv.len()
        }
    }
}
