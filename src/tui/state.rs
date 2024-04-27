use serde_json::json;

use crate::{
    events::{Event, IrcEvents},
    types::{Init, More, Msg, Names, Network, NetworkChannel},
};

pub struct TuiState {
    /// All available networks
    networks: Vec<Network>,
    /// Users in currently active network
    names: Option<Names>,
    /// Id of currently active channel
    active: u32,
    /// index into networks list
    network_idx: usize,
    /// index into channel inside of network
    channel_idx: usize,
    /// List of events
    events: IrcEvents,
}

impl TuiState {
    pub fn new() -> Self {
        Self {
            networks: Vec::new(),
            events: IrcEvents::new(),
            names: None,
            active: 0,
            network_idx: 0,
            channel_idx: 0,
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

    pub fn active(&self) -> u32 {
        self.active
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

    fn update_active(&mut self) {
        let channel = &mut self.networks[self.network_idx].channels[self.channel_idx];
        self.active = channel.id;
        self.events.emit("open", self.active.to_string());
        if !channel.loaded {
            if channel.type_ == "channel" {
                self.events.emit("names", json!({"target": self.active}));
            }

            // TODO: Handle showInActive case in messages
            if let Some(msg) = channel.messages.last() {
                let last_msg_id = msg.id;
                self.events.emit(
                    "more",
                    json!({"target": self.active, "lastId": last_msg_id, "condensed": false}),
                );
            }

            channel.loaded = true;
        }
    }

    pub fn prev_channel(&mut self) {
        if self.channel_idx == 0 {
            if self.network_idx != 0 {
                self.network_idx -= 1;
                self.channel_idx = self.networks[self.network_idx].channels.len() - 1;
            }
        } else {
            self.channel_idx -= 1;
        }

        self.update_active();
    }

    pub fn next_channel(&mut self) {
        if self.channel_idx >= self.networks[self.network_idx].channels.len() - 1 {
            if self.network_idx < self.networks.len() - 1 {
                self.network_idx += 1;
                self.channel_idx = 0;
            }
        } else {
            self.channel_idx += 1;
        }

        self.update_active();
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
            Event::More(more) => self.on_more(more),
            Event::Names(names) => self.on_names(names),
        }
    }

    fn set_selected(&mut self) {
        let mut network_idx = 0;
        let mut channel_idx = 0;

        'outer: for network in &self.networks {
            channel_idx = 0;
            for channel in &network.channels {
                if self.active == channel.id {
                    break 'outer;
                }
                channel_idx += 0;
            }
            network_idx += 1;
        }

        self.network_idx = network_idx;
        self.channel_idx = channel_idx;
    }

    fn on_init(&mut self, init: Init) {
        self.active = init.active;
        self.networks = init.networks;

        self.set_selected();
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

    fn on_more(&mut self, more: More) {
        'outer: for network in &mut self.networks {
            for channel in &mut network.channels {
                if channel.id == more.chan {
                    channel.messages.splice(..0, more.messages);
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
