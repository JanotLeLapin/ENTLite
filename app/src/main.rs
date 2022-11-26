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
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ])
        .split(size);

    let top = Block::default()
        .title("ENTLite")
        .borders(Borders::ALL);

    let bottom = Block::default()
        .title("Messages")
        .borders(Borders::ALL);

    let user = api::user::fetch_userinfo(&app.client).unwrap();
    let unread_count = api::message::fetch_count(&app.client, true).unwrap();

    let mut header = vec![Spans::from(format!("Welcome {}", user.first_name))];
    if unread_count > 0 {
        header.push(Spans::from(format!("You have {} unread messages.", unread_count)));
    }

    let messages: Vec<Spans> = api::message::fetch_messages(&app.client, "/Inbox", 0, false)
        .unwrap()
        .iter()
        .map(|msg| {
            let author = api::user::fetch_person(&app.client, &msg.from).unwrap();
            Spans::from(format!("{}: {}", author.display_name, msg.subject))
        })
        .collect();

    f.render_widget(Paragraph::new(header).block(top), chunks[0]);
    f.render_widget(Paragraph::new(messages).block(bottom), chunks[1]);
}