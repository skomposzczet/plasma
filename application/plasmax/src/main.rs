#[allow(unused)]
mod error;
mod account;
mod api;
mod chats;

use account::Authorized;
use api::Api;
use clap::{Parser, Subcommand};
use error::PlasmaError;
use std::io::Write;
use std::{
    io,
    time::{Duration, Instant},
};
use crate::account::Account;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use itertools::Itertools;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Login to account
    Login {
        #[arg(short, long, help="Mail to login with")]
        mail: String,
    },

    /// Register new account
    Register {
        #[arg(short, long, help="Mail to register with")]
        mail: String,
        #[arg(short, long, help="Unique username")]
        username: String,
    },
}

fn get_password() -> String {
    print!("Password: ");
    std::io::stdout().flush().unwrap();
    let pw = rpassword::read_password().unwrap();
    pw
}

async fn cli_get_accout(cli: Cli, api: &Api) -> Result<Option<Account<Authorized>>, PlasmaError> {
    match &cli.command {
        Some(Commands::Login { mail } ) => {
            let acc = match Account::new(mail.clone()).try_login_token(&api).await {
                Ok(a) => a,
                Err(_) => {
                    let pw = get_password();
                    Account::new(mail.clone()).login(pw, &api).await.unwrap()
                },
            };
            Ok(Some(acc))
        },
        Some(Commands::Register { mail, username } ) => {
            let pw = get_password();
            api.register(mail, username, pw).await?;
            Ok(None)
        },
        None => Ok(None),
    }
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
    current: Option<usize>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
            current: None,
        }
    }

    fn show(&mut self) {
        self.current = match self.current {
            Some(_) => None,
            None => self.state.selected(),
        }
    }

    fn get(&self) -> Option<usize> {
        self.current
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

struct App<'a> {
    items: StatefulList<(&'a str, usize)>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(vec![
                ("Item0", 1),
                ("Item1", 2),
                ("Item2", 1),
                ("Item3", 3),
                ("Item4", 1),
            ]),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), PlasmaError> {
    let api = Api::new();
    let cli = Cli::parse();

    let acc = cli_get_accout(cli, &api).await?;
    if acc.is_none() {
        return Ok(());
    }
    let acc = acc.unwrap();
    let chats = acc.chats(&api).await?;
    println!("{:?}", chats);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(1000);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left => app.items.unselect(),
                        KeyCode::Down => app.items.next(),
                        KeyCode::Up => app.items.previous(),
                        KeyCode::Right => app.items.show(),
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui<B: ratatui::backend::Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(f.size());
    
    let chunks = chunks
        .iter()
        .flat_map(|area| {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(100), // fills remaining space
                    Constraint::Min(5),
                ])
                .split(*area)
                .iter()
                .copied()
                .take(5) // ignore Min(0)
                .collect_vec()
        })
        .collect_vec();

    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let lines = vec![Line::from(i.0)];
            ListItem::new(lines).style(Style::default())
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, chunks[0], &mut app.items.state);

    match app.items.get() {
        Some(cur) => {
            let block = Block::default().title(format!("Current: {}", cur)).borders(Borders::ALL);
            let area = centered_rect(60, 20, f.size());
            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(block, area);
        },
        None => {},
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
