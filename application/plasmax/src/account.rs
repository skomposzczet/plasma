use std::marker::PhantomData;
use bson::oid::ObjectId;
use crate::{api::{Api, body::FindBody, response::Message}, chats::{Chats, get_non_user_id}, keyring::Keyring};
use crate::error::PlasmaError;

pub struct Authorized;
pub struct NotAuthorized;

pub struct Account<State = NotAuthorized> {
    mail: String,
    username: Option<String>,
    id: Option<ObjectId>,
    token: Option<String>,
    state: PhantomData<State>,
    keyring: Keyring,
}

impl Account {
    pub fn new(mail: String) -> Self {
        let keyring = Keyring::new(&mail);
        Account {
            mail,
            username: None,
            id: None,
            token: None,
            state: PhantomData,
            keyring,
        }
    }
}

impl Account<NotAuthorized> {
    pub async fn try_login_token(self, api: &Api) -> Result<Account<Authorized>, PlasmaError> {
        let token = self.keyring.read_token()?;
        let username = api.dashboard(&token).await?;
        let params = FindBody::username(username.clone());
        let id = api.find(&token, params).await?.id;
        let account = Account {
            mail: self.mail.clone(),
            username: Some(username),
            id: Some(id),
            token: Some(token),
            state: PhantomData,
            keyring: Keyring::new(&self.mail),
        };
        account.first_login().await;
        Ok(account)
    }

    pub async fn login(self, password: String, api: &Api) -> Result<Account<Authorized>, PlasmaError> {
        let token = api.login(&self.mail, password).await?;
        self.keyring.save_token(&token)?;
        self.try_login_token(api).await
    }
}

impl Account<Authorized> {
    pub fn token(&self) -> &str {
        self.token.as_ref().unwrap()
    }

    pub fn username(&self) -> &String {
        self.username.as_ref().unwrap()
    }

    pub fn id(&self) -> &ObjectId {
        self.id.as_ref().unwrap()
    }

    pub async fn chats(&self, api: &Api) -> Result<Chats, PlasmaError> {
        let chats = api.chats(self.token()).await?;
        let mut usernames: Vec<String> = Vec::new();
        for chat in chats.iter() {
            let id = get_non_user_id(&chat.users, self.id());
            let params = FindBody::id(id.unwrap());
            let un = api.find(self.token(), params).await.unwrap().username;
            usernames.push(un);
        }
        let chats = Chats::new(chats, usernames, self.id());

        Ok(chats)
    }

    pub async fn chat(&self, api: &Api, member: &str) -> Result<ObjectId, PlasmaError> {
        let chat = api.chat(self.token(), member).await?;
        Ok(chat)
    }

    pub async fn messages(&self, api: &Api, chat_id: &ObjectId) -> Result<Vec<Message> ,PlasmaError> {
        let messages = api.messages(self.token(), chat_id).await?;
        Ok(messages)
    }

    pub async fn first_login(&self) {
        
    }
}
