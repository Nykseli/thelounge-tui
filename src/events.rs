use rust_socketio::{client::Client, ClientBuilder, Payload};
use serde_json::json;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::types;

pub enum Event {
    Init(types::Init),
    Msg(types::Msg),
    Names(types::Names),
}

pub struct IrcEvents {
    events: Arc<Mutex<VecDeque<Event>>>,
    client: Client,
}

impl IrcEvents {
    pub fn new() -> Self {
        let events = Arc::new(Mutex::new(VecDeque::new()));
        let client = create_connection(events.clone());
        Self { events, client }
    }

    pub fn disconnect(&mut self) {
        self.client.disconnect().unwrap();
    }

    /// Get a new event if there's on in the queue
    pub fn event(&mut self) -> Option<Event> {
        self.events.lock().expect("Poisoned lock").pop_front()
    }

    /// Send input to to target channel ID
    pub fn emit_input(&self, input: &str, target: u32) {
        if input.is_empty() {
            return;
        }

        let input = json!({"text": input, "target": target});
        // TODO: error handling
        self.client.emit("input", input).unwrap();
    }
}

fn add_event(events: Arc<Mutex<VecDeque<Event>>>, event: Event) {
    events.lock().expect("Poisoned lock").push_back(event);
}

fn create_connection(events: Arc<Mutex<VecDeque<Event>>>) -> Client {
    let client = {
        let events = events.clone();
        ClientBuilder::new("http://127.0.0.1:9000")
            .namespace("/")
            // .transport_type(rust_socketio::TransportType::Any)
            .on("init", move |data, client| {
                if let Payload::Text(mut data) = data {
                    assert!(data.len() == 1);

                    // this is stupid, I hate it but I don't know how to get the data ownedship otherwise
                    let init: types::Init = serde_json::from_value(data.swap_remove(0)).unwrap();
                    let active_channel = init.active;
                    let type_ = init.active_channel().unwrap().type_.clone();
                    add_event(events.clone(), Event::Init(init));

                    // TODO: handle the open even from server
                    client
                        .emit("open", active_channel.to_string())
                        .expect("Server unreachable");

                    if type_ == "channel" {
                        client
                            .emit("names", json!({"target": active_channel}))
                            .expect("Server unreachable");
                    }
                }
            })
            .on("auth:start", |_, client| {
                let auth = json!({"user":"duck","password":"duck"});
                client
                    .emit("auth:perform", auth)
                    .expect("Server unreachable");
            })
    };

    let client = {
        let events = events.clone();
        client.on("names", move |data, _| {
            if let Payload::Text(mut data) = data {
                assert!(data.len() == 1);

                // this is stupid, I hate it but I don't know how to get the data ownedship otherwise
                let names: types::Names = serde_json::from_value(data.swap_remove(0)).unwrap();
                add_event(events.clone(), Event::Names(names))
            }
        })
    };

    let events = events.clone();
    let client = client
        .on("msg", move |data, _| {
            if let Payload::Text(mut data) = data {
                assert!(data.len() == 1);
                // this is stupid, I hate it but I don't know how to get the data ownedship otherwise
                let msg: types::Msg = serde_json::from_value(data.swap_remove(0)).unwrap();
                add_event(events.clone(), Event::Msg(msg));
            }
        })
        // .on("error", |err, _| panic!("{:#?}", err))
        .on_any(|_event, _payload, _| {
            // println!("{event:#?}");
            // println!("{payload:#?}");
        });

    client.connect().expect("Connection failed")
}
