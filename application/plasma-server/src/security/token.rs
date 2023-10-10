use crate::model::user::User;
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, TokenData};
use warp::{hyper::{HeaderMap, header::AUTHORIZATION}, http::HeaderValue};
use dotenv;
use crate::error::Error;

const TOKEN_DURATION: i64 = 60;
const SECRET_KEY: &str = "SECRET";
const BEARER: &str = "Bearer ";

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn from_user(user: &User) -> Result<Claims, Error> {
        let claims = Claims {
            sub: format!("{:?}", user.id()
                .ok_or(Error::InvalidClaimData("user id is None"))?
            ),
            exp: Utc::now()
                .checked_add_signed(Duration::minutes(TOKEN_DURATION))
                .ok_or(Error::InvalidClaimData("token expiration date exceeded"))?
                .timestamp() as usize
        };

        Ok(claims)
    }

    pub fn sub(self: &Self) -> String {
        self.sub.clone()
    }
}

pub fn create_jwt(user: &User) -> Result<String, Error> {
    let claims = Claims::from_user(&user)?;
    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(get_secret()?
            .as_bytes())
    ).map_err(|err| Error::JWTokenError(err))
}

pub fn decode_jwt(token: &String) -> Result<TokenData::<Claims>, Error> {
    decode::<Claims>(
        token, 
        &DecodingKey::from_secret(get_secret()?.as_bytes()), 
        &Validation::default()
    ).map_err(|err| Error::JWTokenError(err))
}

pub fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Option<String> {
    let header = headers.get(AUTHORIZATION)?;
    let auth_header = std::str::from_utf8(header.as_bytes()).ok()?;

    if !auth_header.starts_with(BEARER) {
        return None;
    }

    let token = auth_header
        .trim_start_matches(BEARER)
        .to_owned();

    Some(token)
}

fn get_secret() -> Result<String, Error> {
    dotenv::var(SECRET_KEY)
        .map_err(|err| Error::EnvError(err))
}
