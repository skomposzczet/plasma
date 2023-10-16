#[allow(unused)]
mod error;
mod account;
mod api;

use account::Authorized;
use api::Api;
use clap::{Parser, Subcommand};
use error::PlasmaError;
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

    Ok(())
}
