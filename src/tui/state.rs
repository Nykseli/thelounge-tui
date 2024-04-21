use crate::{
    events::{Event, IrcEvents},
    types::{ChannelMessage, Init, Msg, Network},
};

pub struct TuiState {
    networks: Vec<Network>,
    events: IrcEvents,
}

impl TuiState {
    pub fn new() -> Self {
        Self {
            networks: Vec::new(),
            events: IrcEvents::new(),
        }
    }

    pub fn events(&self) -> &IrcEvents {
        &self.events
    }

    pub fn networks(&self) -> &[Network] {
        &self.networks
    }

    pub fn messages(&self, channel: u32) -> Option<&[ChannelMessage]> {
        for network in &self.networks {
            for chan in &network.channels {
                if chan.id == channel {
                    return Some(&chan.messages);
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
        }
    }

    fn on_init(&mut self, init: Init) {
        self.networks = init.networks;
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
