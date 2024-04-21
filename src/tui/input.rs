use ratatui::widgets::{Block, Borders, Paragraph};

pub struct InputWidget {}

impl InputWidget {
    pub fn ui(input: &str) -> Paragraph {
        Paragraph::new(input).block(Block::default().borders(Borders::ALL).title("Input"))
    }
}
