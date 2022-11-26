use std::{env, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders},
    Frame,
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide at least 2 arguments: <email> <password>");
    }

    let client = api::auth::login(args.get(1).unwrap(), args.get(2).unwrap()).await.unwrap();

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    app(&mut terminal);

    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();
}

fn app<B: Backend>(terminal: &mut Terminal<B>) {
    loop {
        terminal.draw(|f| ui(f)).unwrap();

        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char('q') => return,
                _ => {},
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();
    let block = Block::default()
        .title("ENTLite")
        .borders(Borders::ALL);
    f.render_widget(block, size);
}