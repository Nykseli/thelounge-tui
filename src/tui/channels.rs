use crate::types::Network;
use ratatui::{
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem},
};

pub struct ChannelsWidget {}

impl ChannelsWidget {
    pub fn ui(networks: &[Network]) -> List<'_> {
        let mut channels: Vec<ListItem> = Vec::new();

        for network in networks {
            channels.push(
                ListItem::new(Span::raw(&network.name))
                    .style(Style::default().add_modifier(Modifier::BOLD)),
            );

            for channel in &network.channels {
                if channel.name != network.name {
                    channels.push(ListItem::new(Span::raw(format!("  {}", channel.name))));
                }
            }

            // empty line to separate channels from eachother
            channels.push(ListItem::new(Span::raw("")));
        }

        List::new(channels).block(Block::default().borders(Borders::ALL).title("Networks"))
    }
}
