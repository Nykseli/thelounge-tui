use ratatui::{
    text::Span,
    widgets::{Block, Borders, List, ListDirection, ListItem},
};

use crate::types::ChannelMessage;

pub struct ChatWidget {}

impl ChatWidget {
    pub fn ui<'a>(title: &'a str, messages: &'a [ChannelMessage]) -> List<'a> {
        let messages: Vec<ListItem> = messages
            .iter()
            .map(|m| {
                let msg = if let (Some(mode), Some(nick)) = (&m.from.mode, &m.from.nick) {
                    format!("    {mode}{nick}")
                } else {
                    "    ~system~".into()
                };

                let msg = if m.type_ != "message" {
                    format!("{msg}: {}", m.type_)
                } else {
                    format!("{msg}: {}", m.text)
                };

                let content = Span::raw(msg);
                ListItem::new(content)
            })
            .rev()
            .collect();

        List::new(messages)
            .direction(ListDirection::BottomToTop)
            .block(Block::default().borders(Borders::ALL).title(title))
    }
}
