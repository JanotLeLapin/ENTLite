use std::{env, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color},
    text::{Span, Spans},
    Frame,
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

struct App {
    me: api::user::Userinfo,
    messages: Vec<(api::message::Message, api::user::Person)>,
    unread_count: u64,
    selected_message: Option<usize>,
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

    app(&mut terminal, App {
        me: api::user::fetch_userinfo(&client).unwrap(),
        messages: api::message::fetch_messages(&client, "/Inbox", 0, false)
            .unwrap()
            .into_iter()
            .map(|msg| {
                let author = api::user::fetch_person(&client, &msg.from).unwrap();
                (msg, author)
            }).collect(),
        unread_count: api::message::fetch_count(&client, true).unwrap(),
        selected_message: Option::None,
    });

    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();
}

fn app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) {
    loop {
        terminal.draw(|f| ui(f, &app)).unwrap();

        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char('q') => return,
                KeyCode::Up => {
                    app.selected_message = Some(app.selected_message.map_or(0, |x| std::cmp::max(x - 1, 0)))
                }
                KeyCode::Down => {
                    app.selected_message = Some(app.selected_message.map_or(1, |x| std::cmp::max(x + 1, 0)))
                }
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

    let mut header = vec![Spans::from(format!("Welcome {}", app.me.first_name))];
    if app.unread_count > 0 {
        header.push(Spans::from(format!("You have {} unread messages.", app.unread_count)));
    }

    let hovered_style = Style::default()
        .fg(Color::Black)
        .bg(Color::White);

    let messages: Vec<Spans> = app.messages
        .iter()
        .enumerate()
        .map(|(i, msg)| {
            let (msg, user) = msg;
            let span = Spans::from(
                Span::styled(
                    format!("{}: {}", user.display_name, msg.subject),
                    if i == app.selected_message.unwrap_or(0) { hovered_style } else { Style::default() }
                )
            );
            span
        })
        .collect();

    f.render_widget(Paragraph::new(header).block(top), chunks[0]);
    f.render_widget(Paragraph::new(messages).block(bottom), chunks[1]);
}