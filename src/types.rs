use serde::{Deserialize, Serialize};

/// Name is similar to User execpt with more detailed information
#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    pub nick: String,
    pub modes: Vec<String>,
    #[serde(rename = "lastMessage")]
    pub last_message: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Names {
    pub id: u64,
    pub users: Vec<Name>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    // TODO: map mode into serverOptions->prefix
    // TODO: these should always exists. Fix parsing of lobby messsage
    pub mode: Option<String>,
    pub nick: Option<String>,
}

impl From<&Name> for User {
    fn from(value: &Name) -> Self {
        // TODO: pick the "highest" mode if multiple exists
        let mode = value.modes.first().cloned();
        Self {
            mode,
            nick: Some(value.nick.clone()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelMessage {
    /// Who has sent the message
    pub from: User,
    pub gecos: Option<String>,
    pub hostmask: Option<String>,
    pub id: u32,
    // TODO: previews
    /// Did you send the the message
    #[serde(rename = "self")]
    pub self_: bool,
    pub text: String,
    // TODO: how to parse and display time?
    pub time: String,
    // TODO: enums for type
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkChannel {
    pub id: u32,
    #[serde(rename = "firstUnread")]
    pub first_unread: i32,
    pub highlight: i32,
    pub key: String,
    pub muted: bool,
    pub name: String,
    pub state: i32,
    pub topic: String,
    #[serde(rename = "totalMessages")]
    pub total_messages: u32,
    // TODO: enums for type
    #[serde(rename = "type")]
    pub type_: String,
    pub unread: i32,
    pub messages: Vec<ChannelMessage>,
    pub users: Vec<User>,
    #[serde(default)]
    pub loaded: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub channels: Vec<NetworkChannel>,
    /// Name of the server / network
    pub name: String,
    /// Username on the server
    pub nick: String,
    // TODO: serverOptions
    // TODO: status
    pub uuid: String,
}

impl Network {
    pub fn channel(&self, id: u32) -> Option<&NetworkChannel> {
        self.channels.iter().find(|c| c.id == id)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Init {
    pub active: u32,
    pub networks: Vec<Network>,
    pub token: String,
}

impl Init {
    pub fn active_channel(&self) -> Option<&NetworkChannel> {
        for network in &self.networks {
            let channel = network.channel(self.active);
            if channel.is_some() {
                return channel;
            }
        }

        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    pub chan: u32,
    pub msg: ChannelMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct More {
    pub chan: u32,
    pub messages: Vec<ChannelMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Join {
    /// Target network uuid
    pub network: String,
    /// Index in the network channels list
    pub index: usize,
    pub chan: NetworkChannel,
}
