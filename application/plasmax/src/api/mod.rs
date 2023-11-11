pub mod body;
pub mod response;
pub mod ws;

use bson::oid::ObjectId;
use reqwest::Client;
use url::Url;
use x3dh::handshake;

const BASE_URL: &'static str = "http://localhost:8000";

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    ReqwestError( #[from] reqwest::Error ),
}

pub struct Api {
    client: Client,
}

impl Api {
    pub fn new() -> Self {
        Api {
            client: Client::new(),
        }
    }

    fn api_path(endpoint: &str) -> Url {
        Url::parse(BASE_URL)
            .unwrap()
            .join(endpoint)
            .unwrap()
    }

    pub async fn login(&self, email: &str, password: String) -> Result<String, ApiError> {
        let url = Self::api_path("login");
        let params = body::LoginBody {
            email: String::from(email),
            password: String::from(password),
        };

        let response = self.client
            .post(url)
            .json(&params)
            .send()
            .await;

        let jwt = response?
            .json::<response::OkResponse<response::LoginResponse>>().await?
            .data.jwtoken
            .clone();

        Ok(jwt)
    }

    pub async fn register(&self, email: &str, username: &str, password: String) -> Result<bool, ApiError> {
        let url = Self::api_path("register");
        let params = body::RegisterBody {
            email: String::from(email),
            username: String::from(username),
            password: String::from(password),
        };

        let response = self.client
            .post(url)
            .json(&params)
            .send()
            .await;

        let message = response?
            .json::<response::OkResponse<response::RegisterResponse>>().await?
            .data.message;

        Ok(message.eq("success"))
    }

    pub async fn dashboard(&self, token: &str) -> Result<String, ApiError> {
        let url = Self::api_path("dashboard");

        let response = self.client
            .get(url)
            .bearer_auth(token)
            .send()
            .await;

        let username = response?
            .json::<response::OkResponse<response::DashboardResponse>>().await?
            .data
            .username;

        Ok(username)
    }

    pub async fn find(&self, token: &str, params: body::FindBody) -> Result<response::User, ApiError> {
        let url = Self::api_path("user");

        let response = self.client
            .post(url)
            .bearer_auth(token)
            .json(&params)
            .send()
            .await;

        let user = response?
            .json::<response::OkResponse<response::FindResponse>>().await?
            .data
            .user;

        Ok(user)
    }

    pub async fn chats(&self, token: &str) -> Result<Vec<response::Chat>, ApiError> {
        let url = Self::api_path("chats");

        let response = self.client
            .get(url)
            .bearer_auth(token)
            .send()
            .await;

        let chats = response?
            .json::<response::OkResponse<response::ChatsResponse>>().await?
            .data
            .chats;

        Ok(chats)
    }

    pub async fn chat(&self, token: &str, member: &str) -> Result<ObjectId, ApiError> {
        let url = Self::api_path("chat");

        let params = body::ChatBody {
            member: String::from(member),
        };

        let response = self.client
            .post(url)
            .bearer_auth(token)
            .json(&params)
            .send()
            .await;

        let chat = response?
            .json::<response::OkResponse<response::ChatResponse>>().await?
            .data
            .chatid;

        Ok(chat)
    }

    pub async fn messages(&self, token: &str, chat_id: &ObjectId) -> Result<Vec<response::Message>, ApiError> {
        let url = Self::api_path("messages");

        let response = self.client
            .post(url)
            .json(chat_id)
            .bearer_auth(token)
            .send()
            .await;

        let messages = response?
            .json::<response::OkResponse<response::MessagesResponse>>().await?
            .data
            .messages;

        Ok(messages)
    }
    
    pub async fn send_bundle(&self, token: &str, bundle: &handshake::RegisterBundle) -> Result<String, ApiError> {
        let url = Self::api_path("bundle");

        let response = self.client
            .post(url)
            .json(&bundle.serialize())
            .bearer_auth(token)
            .send()
            .await;

        let response = response?
            .json::<response::OkResponse<response::SendBundleResponse>>().await?
            .data
            .bundle;

        Ok(response)
    }

    pub async fn get_peer_bundle(&self, token: &str, username: &str) -> Result<handshake::PeerBundle, ApiError> {
        let url = Self::api_path("peer_bundle");

        let response = self.client
            .post(url)
            .json(username)
            .bearer_auth(token)
            .send()
            .await;

        let bundle = response?
            .json::<response::OkResponse<response::PeerBundleResponse>>().await?
            .data
            .bundle
            .deserialize();

        Ok(bundle)
    }

    pub async fn send_initial_message(&self, token: &str, chat_id: ObjectId, message: handshake::InitialMessage) -> Result<(), ApiError> {
        let url = Self::api_path("initial_message");

        let params = body::SendInitialMessageBody {
            chat_id,
            message: message.serialize(),
        };

        let response = self.client
            .post(url)
            .json(&params)
            .bearer_auth(token)
            .send()
            .await;

        response?.json::<response::OkResponse<response::InitialMessageResponse>>().await?;

        Ok(())
    }

    pub async fn get_initial_message(&self, token: &str, chat_id: &ObjectId) -> Result<Option<handshake::InitialMessage>, ApiError> {
        let url = Self::api_path("get_initial_message");

        let response = self.client
            .post(url)
            .json(&chat_id)
            .bearer_auth(token)
            .send()
            .await;

        let message = response?
            .json::<response::OkResponse<response::GetInitialMesssageResponse>>().await?
            .data
            .message;

        let message = message.map(|m| m.deserialize());

        Ok(message)
    }
}
