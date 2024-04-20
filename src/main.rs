use events::IrcEvents;
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{thread, time};

mod events;
mod types;

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}

fn main() {
    let mut events = IrcEvents::new();

    let stdin_channel = spawn_stdin_channel();
    loop {
        if let Ok(key) = stdin_channel.try_recv() {
            println!("Received: {}", key);
            events.disconnect();
            break;
        }

        if let Some(event) = events.event() {
            match event {
                events::Event::Init(init) => {
                    for network in init.networks {
                        println!("In network: {}", network.name);
                        for channel in network.channels {
                            if channel.type_ == "Lobby" {
                                println!("  In lobby: ");
                            } else {
                                println!("  In channel: {}", channel.name);
                            }
                            for message in channel.messages {
                                if let (Some(mode), Some(nick)) =
                                    (message.from.mode, message.from.nick)
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
                events::Event::Msg(msg) => {
                    let message = msg.msg;
                    if let (Some(mode), Some(nick)) = (message.from.mode, message.from.nick) {
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

        sleep(33);
    }
}
