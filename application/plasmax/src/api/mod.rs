mod body;
mod response;

use reqwest::Client;
use url::Url;

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
}
