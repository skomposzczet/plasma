use std::marker::PhantomData;
use bson::oid::ObjectId;
use x3dh::{handshake::{RegisterBundle, OneTimePreKeyPublicBundle, InitialMessage, PeerBundle}, keys::{IdentityKeyPair, KeyPair, SignedPreKeyPair, OneTimeKeyPair, Key, Signature, EphemeralKeyPair}, x3dh_sig, x3dh};
use crate::{api::{Api, body::FindBody, response::Message}, chats::{Chats, get_non_user_id}, keyring::Keyring};
use crate::error::PlasmaError;

struct KeyPack {
    identity: IdentityKeyPair,
    signed: SignedPreKeyPair,
    one_time: Vec<OneTimeKeyPair>,
    signature: Signature,
}

impl KeyPack {
    fn generate(first_index: u16) -> Self {
        let mut rng = rand::rngs::OsRng::default();

        let identity = IdentityKeyPair::generate(&mut rng);
        let signed = SignedPreKeyPair::generate(&mut rng);
        let signature = identity.sign(&signed.public().key().to_sec1_bytes());
        let onetime: Vec<OneTimeKeyPair> = (first_index..first_index+50)
            .map(|index| OneTimeKeyPair::generate(&mut rng).with_index(index))
            .collect();

        KeyPack { identity, signed, one_time: onetime, signature }
    }

}

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
        account.check_first_login(&api).await?;
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

    pub async fn ensure_secret(&self, api: &Api, chat_id: &ObjectId, username: &str) -> Result<(), PlasmaError> {
        if self.keyring.read_secret(username).is_err() {
            match api.get_initial_message(self.token(), &chat_id).await? {
                Some(message) => {
                    self.make_secret_from_initial_messsage(username, message)?;
                },
                None => {
                    let bundle = api.get_peer_bundle(self.token(), username).await?;
                    let message = self.make_secret_from_peer_bundle(bundle, username)?;
                    api.send_initial_message(self.token(), chat_id.clone(), message).await?;
                },
            };
        }
        Ok(())
    }

    fn make_secret_from_initial_messsage(&self, member: &str, message: InitialMessage) -> Result<(), PlasmaError> {
        let identity = self.keyring.read_identity()?;
        let signed = self.keyring.read_signed()?;
        let onetime = self.keyring.read_onetime(message.one_time_idx)?;
        let secret = x3dh(
            &message.identity,
            &signed,
            &message.ephemeral,
            &identity,
            &onetime
            );
        self.keyring.save_secret(member, &secret)?;
        Ok(())
    }

    fn make_secret_from_peer_bundle(&self, bundle: PeerBundle, member: &str) -> Result<InitialMessage, PlasmaError> {
        let identity = self.keyring.read_identity()?;
        let mut rng = rand::rngs::OsRng::default();
        let ephemeral = EphemeralKeyPair::generate(&mut rng);
        let secret = x3dh_sig(
            &bundle.signature, 
            &identity, 
            &bundle.signed_pre, 
            &ephemeral, 
            &bundle.identity, 
            &bundle.one_time_pre.key()
            ).unwrap();
        self.keyring.save_secret(member, &secret)?;
        let message = InitialMessage {
            identity: identity.public().clone(),
            ephemeral: ephemeral.public().clone(),
            one_time_idx: bundle.one_time_pre.index(),
        };
        Ok(message)
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

    pub async fn chat(&self, api: &Api, username: &str) -> Result<ObjectId, PlasmaError> {
        let chat_id = api.chat(self.token(), username).await?;
        self.ensure_secret(api, &chat_id, username).await?;
        Ok(chat_id)
    }

    pub async fn messages(&self, api: &Api, chat_id: &ObjectId) -> Result<Vec<Message>, PlasmaError> {
        let messages = api.messages(self.token(), chat_id).await?;
        Ok(messages)
    }

    pub async fn check_first_login(&self, api: &Api) -> Result<(), PlasmaError> {
        match self.keyring.read_identity() {
            Ok(_) => Ok(()),
            Err(_) => self.register_bundle(&api).await
        }
    }

    pub async fn register_bundle(&self, api: &Api) -> Result<(), PlasmaError> {
        let key_pack = KeyPack::generate(0);
        self.save_key_pack(&key_pack)?;
        self.upload_bundle(&api, key_pack).await?;
        Ok(())
    }

    fn save_key_pack(&self, key_pack: &KeyPack) -> Result<(), PlasmaError> {
        self.keyring.save_identity(&key_pack.identity)?;
        self.keyring.save_signed(&key_pack.signed)?;
        for key in key_pack.one_time.iter() {
            self.keyring.save_onetime(&key)?;
        }
        Ok(())
    }

    async fn upload_bundle(&self, api: &Api, key_pack: KeyPack) -> Result<(), PlasmaError> {
        let bundle = RegisterBundle {
            identity: key_pack.identity.public().clone(),
            signed_pre: key_pack.signed.public().clone(),
            signature: key_pack.signature,
            one_time_pres: key_pack.one_time.iter()
                .map(|key| OneTimePreKeyPublicBundle::from_pair(key))
                .collect(),
        };
        api.send_bundle(self.token(), &bundle).await?;

        Ok(())
    }
}
