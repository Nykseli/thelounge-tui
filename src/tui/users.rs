use crate::types::User;
use ratatui::{
    text::Span,
    widgets::{Block, Borders, List, ListItem},
};

pub struct UsersWidget {}

impl UsersWidget {
    pub fn ui(users: &[User]) -> List<'_> {
        let user_list: Vec<ListItem> = users
            .iter()
            .filter(|usr| usr.nick.is_some())
            .map(|usr| {
                let nick = usr.nick.as_ref().unwrap();
                let nick = if let Some(mode) = &usr.mode {
                    format!("{mode}{nick}")
                } else {
                    nick.into()
                };
                ListItem::new(Span::raw(nick))
            })
            .collect();

        List::new(user_list).block(Block::default().borders(Borders::ALL).title("users"))
    }
}
