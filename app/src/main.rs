use std::{env, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    text::Spans,
    Frame,
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

struct App {
    client: reqwest::blocking::Client,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide at least 2 arguments: <email> <password>");
    }

    let client = api::auth::login(args.get(1).unwrap(), args.get(2).unwrap()).unwrap();

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    app(&mut terminal, &App {
        client,
    });

    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();
}

fn app<B: Backend>(terminal: &mut Terminal<B>, app: &App) {
    loop {
        terminal.draw(|f| ui(f, app)).unwrap();

        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char('q') => return,
                _ => {},
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(size);
    let block = Block::default()
        .title("ENTLite")
        .borders(Borders::ALL);

    let user = api::user::fetch_userinfo(&app.client).unwrap();

    let header = vec![Spans::from(format!("Welcome {}", user.first_name))];

    let paragraph = Paragraph::new(header).block(block);

    f.render_widget(paragraph, chunks[0]);
}