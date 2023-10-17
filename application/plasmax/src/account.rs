use std::{fs::{self, File, create_dir_all}, io::{Error, ErrorKind, Write}, path::PathBuf, marker::PhantomData};
use home::home_dir;
use crate::{api::{Api, body::FindBody}, chats::{Chats, get_non_user_id}};
use crate::error::PlasmaError;

const BASE_PATH: &'static str = ".plasmax";
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

    fn account_path(&self) -> Result<PathBuf, Error> {
        let path = home_dir()
            .ok_or(Error::new(ErrorKind::NotFound, "Impossible to get home directory."))?
            .join(BASE_PATH)
            .join(self.mail.clone());
        create_dir_all(&path)?;
        Ok(path)
    }

    fn token_path(&self) -> Result<PathBuf, Error> {
        let path = self.account_path()?
            .join(TOKEN_FILENAME);
        Ok(path)
    }

    fn read_token(&self) -> Result<String, Error> {
        let path = self.token_path()?;
        let token = fs::read_to_string(path)?;
        Ok(token)
    }

    fn save_token(&self, token: &str) -> Result<(), Error> {
        let path = self.token_path()?;
        let mut file = File::create(path)?;
        file.write_all(token.as_bytes())?;
        Ok(())
    }
}

impl Account<NotAuthorized> {
    pub async fn try_login_token(self, api: &Api) -> Result<Account<Authorized>, PlasmaError> {
        let token = self.read_token()?;
        let username = api.dashboard(&token).await?;
        let account = Account {
            mail: self.mail.clone(),
            username: Some(username),
            token: Some(token),
            state: PhantomData,
        };
        Ok(account)
    }

    pub async fn login(self, password: String, api: &Api) -> Result<Account<Authorized>, PlasmaError> {
        let token = api.login(&self.mail, password).await?;
        self.save_token(&token)?;
        self.try_login_token(api).await
    }
}

impl Account<Authorized> {
    pub fn token(&self) -> &str {
        self.token.as_ref().unwrap()
    }

    pub async fn chats(&self, api: &Api) -> Result<Chats, PlasmaError> {
        let chats = api.chats(self.token()).await?;
        let params = FindBody::username(self.username.clone().unwrap());
        let ownid = api.find(self.token(), params).await?.id;

        let mut usernames: Vec<String> = Vec::new();
        for chat in chats.iter() {
            let id = get_non_user_id(&chat.users, &ownid);
            let p = FindBody::id(id.unwrap());
            let un = api.find(self.token(), p).await.unwrap().username;
            usernames.push(un);
        }
        let chats = Chats::new(chats, usernames, ownid);

        Ok(chats)
    }
}
