use crate::types::Name;
use ratatui::{
    text::Span,
    widgets::{Block, Borders, List, ListItem},
};

pub struct UsersWidget {}

impl UsersWidget {
    pub fn ui(users: &[Name]) -> List<'_> {
        let mut user_list: Vec<ListItem> = Vec::new();

        for user in users {
            user_list.push(ListItem::new(Span::raw(&user.nick)));
        }

        List::new(user_list).block(Block::default().borders(Borders::ALL).title("users"))
    }
}
