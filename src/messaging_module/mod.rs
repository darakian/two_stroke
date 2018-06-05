#[cfg(test)]
mod tests {
    use messaging_module::{Message, Omnibus};
    use std::thread;

    #[test]
    fn multi_read() {
        // let mut mb = Omnibus::new("bus");
        // mb.publish(Message::new("1",2));
        // mb.publish(Message::new("1",2));
        // mb.publish(Message::new("1",2));
        // assert_eq!(mb.check_channel_len(), 3);
        // let h1 = thread::spawn(move || {
        //     mb.publish(Message::new("2",2));
        //     mb.publish(Message::new("2",2));
        //     mb.publish(Message::new("2",2));
        //     mb
        // });
        // let mut mb = h1.join().unwrap();
        // let h2 = thread::spawn(move || {
        //     mb.publish(Message::new("3",2));
        //     mb.publish(Message::new("3",2));
        //     mb.publish(Message::new("3",2));
        //     mb
        // });
        // let mut mb = h2.join().unwrap();
        // let h3 = thread::spawn(move || {
        //     mb.publish(Message::new("4",2));
        //     mb.publish(Message::new("4",2));
        //     mb.publish(Message::new("4",2));
        //     mb
        // });
        // let mut mb = h3.join().unwrap();
        // mb.publish(Message::new("bus",2));
        //
        // let (send1, recv1) = mb.join(7).unwrap();
        // mb.subscribe("3", 7).unwrap();
        // let (send2, recv2) = mb.join(8).unwrap();
        // mb.subscribe("3", 8).unwrap();
        // mb.do_messaging();
        // for element in recv1.try_iter(){
        //     println!(">>>recv1: {:?}", element);
        // }
        // for element in recv2.try_iter(){
        //     println!(">>>recv:2 {:?}", element);
        // }
    }
}



pub mod omnibus {
extern crate crossbeam_channel;
use self::crossbeam_channel::unbounded;
use std::collections::hash_map::{HashMap, Entry};

    #[derive(Debug, Clone)]
    pub struct Message{
        publish_tag: String,
        publisher: u64,
        payload: Option<OmniPayload>
    }

#[derive(Debug, Clone)]
 enum OmniPayload {
    Quit,
    Subscribe(String),
    Tick(u64),
    // Move {publish_tag: String, object_tag: String, x: i32, y: i32 },
    // RNG {publish_tag: String, value: u16},
    // Write {publish_tag: String, Message: String},
    // ChangeColor {publish_tag: String, object_tag: String, r: i32, g: i32, b: i32},
    // Sprite {publish_tag: String, object_tag: String,x: i32, y: i32, pixels: Vec<u8>}
}

    impl Message {
        pub fn new_sub(to: &str, from: u64, subscribe_string: &str) -> Self{
            Message{publish_tag: to.to_string(), publisher: from, payload: Some(OmniPayload::Subscribe(subscribe_string.to_string()))}
        }
    }

    pub struct Omnibus{
        bus_id: String,
        global_recv: crossbeam_channel::Receiver<Message>,
        global_send: crossbeam_channel::Sender<Message>,
        subscribers:  HashMap<u64, (crossbeam_channel::Sender<Message>)>,
        feeds: HashMap<String, Vec<crossbeam_channel::Sender<Message>>>
    }

    impl Omnibus{
        pub fn new(bus_id: &str) -> Self{
            let (send, receive) = unbounded::<Message>();
            let mut bus = Omnibus{bus_id: bus_id.to_string(), global_recv: receive, global_send: send, subscribers: HashMap::new(), feeds: HashMap::new()};
            let (bus_self_tx, bus_self_rx) = bus.join(0).unwrap();
            bus
        }

        pub fn join(&mut self, component_id: u64) -> Result<(crossbeam_channel::Sender<Message>, crossbeam_channel::Receiver<Message>), &str>{
            let (send, receive) = unbounded::<Message>();
                    match self.subscribers.entry(component_id) {
                        Entry::Vacant(es) => {es.insert(send.clone());},
                        Entry::Occupied(mut e) => {return Err("Sub_ID in use");}
                        }
            Ok((self.global_send.clone(), receive))
        }

        fn subscribe(&mut self, sub_tag: &str, component_id: u64) -> Result<(), &str>{
            let relevant_channel = self.subscribers.get(&component_id).unwrap().clone();
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
                let msg = self.global_recv.recv().unwrap();
                println!("{:?}", msg);
                println!("meg_publish tag: {:?}  bus tag: {:?}  same? {:?}", msg.publish_tag, self.bus_id, msg.publish_tag==self.bus_id);
                if self.subscribers.get(&msg.publisher)==None {/*drop(msg); continue;*/}
                if msg.publish_tag == self.bus_id{
                    let pub_tag = msg.publish_tag.clone();
                    let pub_er = msg.publisher;
                    match msg.payload{
                        Some(kind) => {
                        println!("HERE {:?}", kind);
                        match kind {
                            OmniPayload::Quit => return,
                            OmniPayload::Subscribe(sub_tag) => {
                                self.subscribe(&sub_tag, pub_er);
                                println!("Also here with {}", sub_tag);
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
                                //println!("Sending {:?} to {:?}", msg);
                            x.send(msg.clone()).unwrap()})},
                        None => {drop(msg)}
                    }
                }

            }
        }


        //Testing methods
        pub fn publish(&mut self, m: Message) -> (){
            self.global_send.send(m).unwrap();
        }

        pub fn check_channel_len(&self) -> usize{
            self.global_recv.len()
        }
    }
}
