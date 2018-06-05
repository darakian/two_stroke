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

use std::sync::Mutex;
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
        subscribers:  Mutex<HashMap<u64, (crossbeam_channel::Sender<Message>)>>,
        feeds: Mutex<HashMap<String, Vec<crossbeam_channel::Sender<Message>>>>
    }

    impl Omnibus{
        pub fn new(bus_id: &str) -> Self{
            let (send, receive) = unbounded::<Message>();
            let mut bus = Omnibus{bus_id: bus_id.to_string(), global_recv: receive, global_send: send, subscribers: Mutex::new(HashMap::new()), feeds: Mutex::new(HashMap::new())};
            let (bus_self_tx, bus_self_rx) = bus.join(0).unwrap();
            bus
        }

        pub fn join(&mut self, component_id: u64) -> Result<(crossbeam_channel::Sender<Message>, crossbeam_channel::Receiver<Message>), &str>{
            let (send, receive) = unbounded::<Message>();
            match self.subscribers.get_mut(){
                Ok(exclusive_subscribers) => {
                    match exclusive_subscribers.entry(component_id) {
                        Entry::Vacant(es) => {es.insert(send.clone());},
                        Entry::Occupied(mut e) => {return Err("Sub_ID in use");}
                        }
                },
                Err(e) => {println!("{:?}", e);return Err("Poison error")}
            }
            Ok((self.global_send.clone(), receive))
        }

        fn subscribe(&mut self, sub_tag: &str, component_id: u64) -> Result<(), &str>{
            match self.subscribers.get_mut(){
                Ok(exclusive_subscribers) => {
                    match self.feeds.get_mut(){
                        Ok(exclusive_feeds) => {
                            exclusive_feeds.entry(sub_tag.to_string())
                            .and_modify(|vec| {
                                if vec.contains(exclusive_subscribers.get(&component_id).unwrap()) {/*Should handle this case. TODO*/}
                                else {vec.push(exclusive_subscribers.get(&component_id).unwrap().clone());}
                             })
                            .or_insert({let mut vec = Vec::new(); vec.push(exclusive_subscribers.get(&component_id).unwrap().clone()); vec});
                        },
                        Err(e) => {}
                    }
                },
                Err(e) => {println!("{:?}", e);return Err("Poison error")}
            }
            Ok(())
        }

        pub fn do_messaging(&mut self) {
            loop {
                let msg = self.global_recv.recv().unwrap();
                println!("{:?}", msg);
                println!("meg_publish tag: {:?}  bus tag: {:?}  same? {:?}", msg.publish_tag, self.bus_id, msg.publish_tag==self.bus_id);
                match self.subscribers.get_mut() {
                    Ok(exclusive_subscribers) => if exclusive_subscribers.get(&msg.publisher)==None {/*drop(msg); continue;*/}
                    Err(e) => {println!("{:?}", e);}
                }
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
                    match self.feeds.get_mut(){
                            Ok(exclusive_feeds) => {
                                    match exclusive_feeds.get(&msg.publish_tag){
                                        Some(feed_subscribers) => {
                                            feed_subscribers.iter().for_each(|x| {
                                                //println!("Sending {:?} to {:?}", msg);
                                            x.send(msg.clone()).unwrap()})},
                                        None => {drop(msg)}
                                    }
                            },
                            Err (e) => {}
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
