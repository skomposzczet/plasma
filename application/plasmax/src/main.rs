#[allow(unused)]
mod account;
mod api;

use api::Api;
use clap::{Parser, Subcommand};
use std::io::Write;

use crate::account::Account;

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

#[tokio::main]
async fn main() {
    let api = Api::new();
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Login { mail } ) => {
            let acc = Account::new(mail.clone());
            let acc = match acc.try_login_token() {
                Ok(a) => a,
                Err(_) => {
                    let pw = get_password();
                    println!("ml={}, pw={}", mail, pw);
                    acc.login(pw, &api).await.unwrap()
                },
            };
        },
        Some(Commands::Register { mail, username } ) => {
            let pw = get_password();
            println!("ml={}, un={}, pw={}", mail, username, pw);
        },
        None => {},
    }
}
