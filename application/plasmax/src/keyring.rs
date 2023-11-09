use std::{path::PathBuf, io::{Error, ErrorKind, Write}, fs::{create_dir_all, self, File}};

use home::home_dir;

const BASE_PATH: &'static str = ".plasmax";
const TOKEN_FILENAME: &'static str = "token";

pub struct Keyring {
    mail: String,
}

impl Keyring {
    pub fn new(mail: &str) -> Self {
        Keyring { mail: mail.to_owned() }
    }

    pub fn read_token(&self) -> Result<String, Error> {
        let path = self.token_path()?;
        let token = fs::read_to_string(path)?;
        Ok(token)
    }

    pub fn save_token(&self, token: &str) -> Result<(), Error> {
        let path = self.token_path()?;
        let mut file = File::create(path)?;
        file.write_all(token.as_bytes())?;
        Ok(())
    }

    fn token_path(&self) -> Result<PathBuf, Error> {
        let path = self.account_path()?
            .join(TOKEN_FILENAME);
        Ok(path)
    }

    fn account_path(&self) -> Result<PathBuf, Error> {
        let path = home_dir()
            .ok_or(Error::new(ErrorKind::NotFound, "Impossible to get home directory."))?
            .join(BASE_PATH)
            .join(self.mail.clone());
        create_dir_all(&path)?;
        Ok(path)
    }
}
