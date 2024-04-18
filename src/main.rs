use rust_socketio::{ClientBuilder, Payload};
use serde_json::json;

mod types;

fn main() {
    // get a socket that is connected to the admin namespace
    let socket = ClientBuilder::new("http://127.0.0.1:9000")
        .namespace("/")
        // .transport_type(rust_socketio::TransportType::Any)
        .on("init", |data, _| {
            if let Payload::Text(mut data) = data {
                assert!(data.len() == 1);

                // this is stupid, I hate it but I don't know how to get the data ownedship otherwise
                let init: types::Init = serde_json::from_value(data.swap_remove(0)).unwrap();

                for network in init.networks {
                    println!("In network: {}", network.name);
                    for channel in network.channels {
                        if channel.type_ == "Lobby" {
                            println!("  In lobby: ");
                        } else {
                            println!("  In channel: {}", channel.name);
                        }
                        for message in channel.messages {
                            if let (Some(mode), Some(nick)) = (message.from.mode, message.from.nick)
                            {
                                print!("    {mode}{nick}");
                            } else {
                                print!("    ~system~");
                            }

                            if message.type_ != "message" {
                                println!(": {}", message.type_)
                            } else {
                                println!(": {}", message.text)
                            }
                        }
                    }
                }
            }
        })
        .on("auth:start", |_, client| {
            let auth = json!({"user":"duck","password":"duck"});
            client
                .emit("auth:perform", auth)
                .expect("Server unreachable");
        })
        // .on("error", |err, _| panic!("{:#?}", err))
        .on_any(|_event, _payload, _| {
            // println!("{event:#?}");
            // println!("{payload:#?}");
        })
        .connect()
        .expect("Connection failed");

    std::thread::sleep(std::time::Duration::from_secs(5));

    socket.disconnect().expect("Disconnect failed")
}
