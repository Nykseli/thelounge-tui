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

use self::{
    channels::ChannelsWidget, chat::ChatWidget, input::InputWidget, state::TuiState,
    users::UsersWidget,
};

mod channels;
mod chat;
mod input;
mod state;
mod users;

struct Buffer {
    text: String,
    pos: usize,
}

impl Buffer {
    fn new() -> Self {
        Self {
            text: "".into(),
            pos: 0,
        }
    }

    fn clear(&mut self) {
        self.text.truncate(0);
        self.pos = 0;
    }

    fn add(&mut self, c: char) {
        self.text.insert(self.pos, c);
        self.pos += 1;
    }

    fn next(&mut self) {
        if self.pos < self.text.chars().count() {
            self.pos += 1;
        }
    }

    fn prev(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    fn backspace(&mut self) {
        if !self.text.is_empty() {
            self.pos -= 1;
            self.text.remove(self.pos);
        }
    }
}

struct TuiApp {
    input_buffer: Buffer,
    state: TuiState,
    show_users: bool,
    show_channels: bool,
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            state: TuiState::new(),
            input_buffer: Buffer::new(),
            show_users: false,
            show_channels: false,
        }
    }

    fn key_event(&mut self, key: KeyEvent) {
        if key.kind != event::KeyEventKind::Press {
            return;
        }

        if key.modifiers.contains(KeyModifiers::ALT) {
            match key.code {
                KeyCode::Up => self.state.prev_channel(),
                KeyCode::Down => self.state.next_channel(),
                KeyCode::Char('b') => self.show_channels = !self.show_channels,
                KeyCode::Char('v') => self.show_users = !self.show_users,
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Char(c) => self.input_buffer.add(c),
                KeyCode::Backspace => {
                    self.input_buffer.backspace();
                }
                KeyCode::Left => {
                    self.input_buffer.prev();
                }
                KeyCode::Right => {
                    self.input_buffer.next();
                }
                KeyCode::Enter => {
                    self.state
                        .handle_input(&self.input_buffer.text, self.state.active());
                    self.input_buffer.clear();
                }
                _ => {}
            }
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
    let channelw = if app.show_channels { 10 } else { 0 };

    let usersw = if app.show_users { 10 } else { 0 };

    let chatw = 100 - channelw - usersw;

    let horizontal = Layout::horizontal([
        // Channel list
        Constraint::Percentage(channelw),
        // Chat
        Constraint::Percentage(chatw),
        // Channel members
        Constraint::Percentage(usersw),
    ]);

    let vertical = Layout::vertical([Constraint::Percentage(80), Constraint::Percentage(20)]);
    let [channels, chat, members] = horizontal.areas(frame.size());
    let [messages, input] = vertical.areas(chat);

    if app.show_channels {
        frame.render_widget(
            ChannelsWidget::ui(app.state.networks(), app.state.active()),
            channels,
        );
    }

    if let Some(channel) = app.state.channel(app.state.active()) {
        frame.render_widget(ChatWidget::ui(&channel.name, &channel.messages), messages);
        if app.show_users {
            frame.render_widget(UsersWidget::ui(&channel.users), members);
        }
    } else {
        frame.render_widget(tmp_area("messages"), messages);
        frame.render_widget(tmp_area("members"), members);
    }

    frame.render_widget(InputWidget::ui(&app.input_buffer.text), input);
    frame.set_cursor(input.x + 1 + app.input_buffer.pos as u16, input.y + 1);
}
