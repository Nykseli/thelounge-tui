use serde_json::json;

use crate::{
    events::{Event, IrcEvents},
    types::{Init, Join, More, Msg, Names, Network, NetworkChannel},
};

pub struct TuiState {
    /// All available networks
    networks: Vec<Network>,
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
            active: 0,
            network_idx: 0,
            channel_idx: 0,
        }
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

    pub fn handle_input(&mut self, input: &str, target: u32) {
        if !self.handle_commad(input) {
            self.events.emit_input(input, target);
        }
    }

    /// Check if input has a command that can be handled on the client side.
    /// returns true if command is handled on the client side.
    fn handle_commad(&mut self, input: &str) -> bool {
        let inputs: Vec<&str> = input.split_whitespace().collect();
        if inputs[0] == "/join" && inputs.len() >= 2 {
            // check if we already are in the channel and jump into it
            let name = inputs[1];
            if let Some(channel) = self.networks[self.network_idx]
                .channels
                .iter()
                .find(|c| c.name == name)
            {
                self.active = channel.id;
                self.set_selected();
                self.update_active();
                return true;
            }
        }

        false
    }

    fn channel_mut(&mut self, channel: u32) -> Option<&mut NetworkChannel> {
        for network in &mut self.networks {
            for chan in &mut network.channels {
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
            Event::Join(join) => self.on_join(join),
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
                channel_idx += 1;
            }
            network_idx += 1;
        }

        self.network_idx = network_idx;
        self.channel_idx = channel_idx;
    }

    fn on_init(&mut self, init: Init) {
        self.active = init.active;
        self.networks = init.networks;
        if let Some(channel) = self.channel_mut(self.active) {
            channel.loaded = true;
        }

        self.set_selected();
    }

    fn on_join(&mut self, join: Join) {
        if let Some(network) = self.networks.iter_mut().find(|n| n.uuid == join.network) {
            self.active = join.chan.id;

            if join.index >= network.channels.len() {
                network.channels.push(join.chan);
            } else {
                network.channels.insert(join.index, join.chan);
            }
        }

        self.set_selected();
    }

    fn on_names(&mut self, names: Names) {
        if let Some(channel) = self.channel_mut(names.id as u32) {
            channel.users = names.users.iter().map(From::from).collect();
        }
    }

    fn on_msg(&mut self, msg: Msg) {
        if let Some(channel) = self.channel_mut(msg.chan) {
            channel.messages.push(msg.msg);
        }
    }

    fn on_more(&mut self, more: More) {
        if let Some(channel) = self.channel_mut(more.chan) {
            channel.messages.splice(..0, more.messages);
        }
    }
}

impl Drop for TuiState {
    fn drop(&mut self) {
        self.events.disconnect();
    }
}
