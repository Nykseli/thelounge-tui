use crate::{
    events::{Event, IrcEvents},
    types::{Init, Msg, Names, Network, NetworkChannel},
};

pub struct TuiState {
    /// All available networks
    networks: Vec<Network>,
    /// Users in currently active network
    names: Option<Names>,
    /// List of events
    events: IrcEvents,
}

impl TuiState {
    pub fn new() -> Self {
        Self {
            networks: Vec::new(),
            events: IrcEvents::new(),
            names: None,
        }
    }

    pub fn events(&self) -> &IrcEvents {
        &self.events
    }

    pub fn names(&self) -> &Option<Names> {
        &self.names
    }

    pub fn networks(&self) -> &[Network] {
        &self.networks
    }

    pub fn channel(&self, channel: u32) -> Option<&NetworkChannel> {
        for network in &self.networks {
            for chan in &network.channels {
                if chan.id == channel {
                    return Some(chan);
                }
            }
        }
        None
    }

    /// Check for new events and update state accordingly
    pub fn update(&mut self) {
        let event = if let Some(event) = self.events.event() {
            event
        } else {
            return;
        };

        match event {
            Event::Init(init) => self.on_init(init),
            Event::Msg(msg) => self.on_msg(msg),
            Event::Names(names) => self.on_names(names),
        }
    }

    fn on_init(&mut self, init: Init) {
        self.networks = init.networks;
    }

    fn on_names(&mut self, names: Names) {
        self.names = Some(names)
    }

    fn on_msg(&mut self, msg: Msg) {
        'outer: for network in &mut self.networks {
            for channel in &mut network.channels {
                if channel.id == msg.chan {
                    channel.messages.push(msg.msg);
                    break 'outer;
                }
            }
        }
    }
}

impl Drop for TuiState {
    fn drop(&mut self) {
        self.events.disconnect();
    }
}
