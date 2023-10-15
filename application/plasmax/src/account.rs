use std::{fs, io::{Error, ErrorKind}, path::PathBuf, marker::PhantomData};
use home::home_dir;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccountError {
    #[error(transparent)]
    NoTokenError( #[from] Error ),
}

const TOKEN_PATH: &'static str = ".plasmax/token";
const TOKEN_FILENAME: &'static str = "token";

pub struct Authorized;
pub struct NotAuthorized;

pub struct Account<State = NotAuthorized> {
    mail: String,
    username: Option<String>,
    token: Option<String>,
    state: PhantomData<State>,
}

impl Account {
    pub fn new(mail: String) -> Self {
        Account {
            mail,
            username: None,
            token: None,
            state: PhantomData,
        }
    }
}

impl Account<NotAuthorized> {
    pub fn try_login_token(self) -> Result<Account<Authorized>, AccountError> {
        let token = Some(self.read_token()?);
        let account = Account {
            mail: self.mail,
            username: Some(String::new()),
            token,
            state: PhantomData,
        };
        Ok(account)
    }

    fn token_path(&self) -> Result<PathBuf, Error> {
        let path = home_dir()
            .ok_or(Error::new(ErrorKind::NotFound, "Impossible to get home directory."))?
            .join(TOKEN_PATH)
            .join(self.mail.clone())
            .join(TOKEN_FILENAME);
        Ok(path)
    }

    fn read_token(&self) -> Result<String, Error> {
        let path = self.token_path()?;
        let token = fs::read_to_string(path)?;
        Ok(token)
    }
}

impl Account<Authorized> {
    pub fn token(&self) -> &str {
        self.token.as_ref().unwrap()
    }
}
