use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
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

use self::{channels::ChannelsWidget, chat::ChatWidget, input::InputWidget, state::TuiState};

mod channels;
mod chat;
mod input;
mod state;

struct TuiApp {
    input_buffer: String,
    state: TuiState,
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            state: TuiState::new(),
            input_buffer: String::new(),
        }
    }

    fn key_event(&mut self, key: KeyEvent) {
        if key.kind != event::KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char(c) => self.input_buffer.push(c),
            // 2 is the test server #foobar channel
            KeyCode::Enter => {
                self.state.events().emit_input(&self.input_buffer, 2);
                self.input_buffer.truncate(0);
            }
            _ => {}
        }
    }
}

pub fn run_tui() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    let mut app = TuiApp::new();

    while !should_quit {
        terminal.draw(|frame| ui(frame, &app))?;
        should_quit = handle_events(&mut app)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn handle_events(app: &mut TuiApp) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press
                && key.modifiers.contains(KeyModifiers::CONTROL)
                && key.code == KeyCode::Char('q')
            {
                return Ok(true);
            }

            app.key_event(key)
        }
    }

    app.state.update();

    Ok(false)
}

fn tmp_area(name: &str) -> impl Widget {
    Paragraph::new(format!("{name} para")).block(
        Block::default()
            .title(format!("{name} title"))
            .borders(Borders::ALL),
    )
}

fn ui(frame: &mut Frame, app: &TuiApp) {
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

    frame.render_widget(ChannelsWidget::ui(app.state.networks()), channels);
    frame.render_widget(tmp_area("members"), members);
    // 2 is the id of the test #foobar channel
    if let Some(msgs) = app.state.messages(2) {
        frame.render_widget(ChatWidget::ui("#foobar", msgs), messages);
    } else {
        frame.render_widget(tmp_area("messages"), messages);
    }

    frame.render_widget(InputWidget::ui(&app.input_buffer), input);
}
