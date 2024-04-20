use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    // TODO: map mode into serverOptions->prefix
    // TODO: these should always exists. Fix parsing of lobby messsage
    pub mode: Option<String>,
    pub nick: Option<String>,
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
    // TODO: users list
    pub messages: Vec<ChannelMessage>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Init {
    pub active: i32,
    pub networks: Vec<Network>,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    pub chan: u32,
    pub msg: ChannelMessage,
}
