use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame, Terminal,
};
use std::io::{self, stdout};

use self::{chat::ChatWidget, state::TuiState};

mod chat;
mod state;

pub fn run_tui() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    let mut state = TuiState::new();

    while !should_quit {
        terminal.draw(|frame| ui(frame, &state))?;
        should_quit = handle_events(&mut state)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
fn handle_events(state: &mut TuiState) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }

    state.update();

    Ok(false)
}

fn tmp_area(name: &str) -> impl Widget {
    Paragraph::new(format!("{name} para")).block(
        Block::default()
            .title(format!("{name} title"))
            .borders(Borders::ALL),
    )
}

fn ui(frame: &mut Frame, state: &TuiState) {
    let horizontal = Layout::horizontal([
        // Channel list
        Constraint::Percentage(10),
        // Chat
        Constraint::Percentage(80),
        // Channel members
        Constraint::Percentage(10),
    ]);
    let vertical = Layout::vertical([Constraint::Percentage(80), Constraint::Percentage(20)]);
    let [channels, chat, members] = horizontal.areas(frame.size());
    let [messages, input] = vertical.areas(chat);

    frame.render_widget(tmp_area("channels"), channels);
    frame.render_widget(tmp_area("members"), members);
    // 2 is the id of the test #foobar channel
    if let Some(msgs) = state.messages(2) {
        frame.render_widget(ChatWidget::ui("#foobar", msgs), messages);
    } else {
        frame.render_widget(tmp_area("messages"), messages);
    }

    frame.render_widget(tmp_area("input"), input);
}
