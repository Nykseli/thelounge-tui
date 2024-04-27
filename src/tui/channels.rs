use crate::types::Network;
use ratatui::{
    style::{Modifier, Style, Stylize},
    text::Span,
    widgets::{Block, Borders, List, ListItem},
};

pub struct ChannelsWidget {}

impl ChannelsWidget {
    pub fn ui(networks: &[Network], active: u32) -> List<'_> {
        let mut channels: Vec<ListItem> = Vec::new();

        for network in networks {
            for channel in &network.channels {
                let (text, style) = if channel.type_ == "lobby" {
                    (
                        channel.name.clone(),
                        Style::default().add_modifier(Modifier::BOLD),
                    )
                } else {
                    (format!("  {}", channel.name), Style::default())
                };

                let style = if channel.id == active {
                    style.reversed()
                } else {
                    style
                };

                channels.push(ListItem::new(Span::raw(text)).style(style));
            }

            // empty line to separate channels from eachother
            channels.push(ListItem::new(Span::raw("")));
        }

        List::new(channels).block(Block::default().borders(Borders::ALL).title("Networks"))
    }
}
