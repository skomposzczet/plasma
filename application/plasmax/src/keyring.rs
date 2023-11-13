use std::{path::PathBuf, io::{Error, ErrorKind, Write, Read}, fs::{create_dir_all, self, File}};

use home::home_dir;
use x3dh::keys::{X3dhSharedSecret, IdentityKeyPair, KeyPair, SignedPreKeyPair, OneTimeKeyPair};

const BASE_PATH: &'static str = ".plasmax";
const TOKEN_FILENAME: &'static str = "token";
const KEYS_DIR: &'static str = "keys";
const SECRET_DIR: &'static str = "chat_secret";

enum KeyType {
    Identity,
    Signed,
    OneTime(u16),
}

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

    pub fn read_identity(&self) -> Result<IdentityKeyPair, Error> {
        self.read_key(KeyType::Identity)
    }

    pub fn save_identity(&self, key: &IdentityKeyPair) -> Result<(), Error> {
        self.save_key(KeyType::Identity, key)
    }

    pub fn read_signed(&self) -> Result<SignedPreKeyPair, Error> {
        self.read_key(KeyType::Signed)
    }

    pub fn save_signed(&self, key: &SignedPreKeyPair) -> Result<(), Error> {
        self.save_key(KeyType::Signed, key)
    }

    pub fn read_onetime(&self, idx: u16) -> Result<OneTimeKeyPair, Error> {
        self.read_key(KeyType::OneTime(idx))
    }

    pub fn save_onetime(&self, key: &OneTimeKeyPair) -> Result<(), Error> {
        self.save_key(KeyType::OneTime(key.index()), key)
    }


    fn read_key<K: KeyPair>(&self, key_type: KeyType) -> Result<K, Error> {
        let path = self.key_path(key_type)?;
        let mut file = File::open(path)?;
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;
        let key = K::from_bytes(&buffer);
        Ok(key)
    }

    fn save_key<K: KeyPair>(&self, key_type: KeyType, key: &K) -> Result<(), Error>{
        let path = self.key_path(key_type)?;
        let mut file = File::create(path)?;
        file.write_all(&key.to_bytes())?;
        Ok(())
    }


    fn key_path(&self, key_type: KeyType) -> Result<PathBuf, Error> {
        let path = self.account_path()?
            .join(KEYS_DIR);
        create_dir_all(&path)?;
        let filename = match key_type {
            KeyType::Identity => String::from("identity"),
            KeyType::Signed => String::from("signed"),
            KeyType::OneTime(idx) => format!("onetime_{}", idx)
        };
        let path = path.join(&filename);
        Ok(path)
    }

    pub fn read_secret(&self, username: &str) -> Result<X3dhSharedSecret, Error> {
        let path = self.secret_path(username)?;
        let mut file = File::open(path)?;
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;
        let secret = X3dhSharedSecret::from_bytes(&buffer);
        Ok(secret)
    }

    pub fn save_secret(&self, username: &str, secret: &X3dhSharedSecret) -> Result<(), Error>{
        let path = self.secret_path(username)?;
        let mut file = File::create(path)?;
        file.write_all(secret.to_bytes())?;
        Ok(())
    }

    fn secret_path(&self, username: &str) -> Result<PathBuf, Error> {
        let path = self.account_path()?
            .join(SECRET_DIR);
        create_dir_all(&path)?;
        let path = path.join(username);
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
