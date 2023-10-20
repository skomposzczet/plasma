#[allow(unused)]
mod error;
mod account;
mod api;
mod chats;
mod tui;

use account::Authorized;
use api::Api;
use ratatui::Terminal;
use tui::app::App;
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
use ratatui::prelude::{Backend, CrosstermBackend};

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
    let app = App::new(chats.chats.clone());
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
        terminal.draw(|f| tui::ui::ui(f, &mut app))?;

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

