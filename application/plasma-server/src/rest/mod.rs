mod user;

use std::{sync::Arc, convert::Infallible};
use serde::Serialize;
use serde_json::json;
use warp::{reply::Json, Rejection, Filter, hyper::{HeaderMap, StatusCode}, http::HeaderValue, Reply};
use crate::{model, error::AuthorizationError};
use crate::error;
use crate::{security::token::{jwt_from_header, decode_jwt}, model::Db};

#[derive(Debug, Clone)]
pub struct WebErrorMessage {
    kind: &'static str,
    message: String,
    status_code: StatusCode,
}
impl warp::reject::Reject for WebErrorMessage {}

impl WebErrorMessage {
    pub fn unknown() -> WebErrorMessage {
        WebErrorMessage{
            kind: "Unkown error", 
            message: String::from("unknown"),
            status_code: StatusCode::BAD_REQUEST
        }
    }
    pub fn rejection(kind: &'static str, message: String, status_code: StatusCode) -> warp::Rejection {
        warp::reject::custom(WebErrorMessage {kind, message, status_code})
    }
}

impl From<error::Error> for warp::Rejection {
    fn from(other: error::Error) -> Self {
        WebErrorMessage::rejection(
            "error::Error",
            format!("{}", other),
            StatusCode::BAD_REQUEST,
        )
    }
}

impl From<model::Error> for warp::Rejection {
    fn from(other: model::Error) -> Self {
        WebErrorMessage::rejection(
            "model::Error",
            format!("{}", other),
            StatusCode::BAD_REQUEST,
        )
    }
}

impl From<error::BsonError> for warp::Rejection {
    fn from(other: error::BsonError) -> Self {
        WebErrorMessage::rejection(
            "error::BsonError",
            format!("{}", other),
            StatusCode::BAD_REQUEST,
        )
    }
}

impl From<error::AuthorizationError> for warp::Rejection {
    fn from(other: error::AuthorizationError) -> Self {
        WebErrorMessage::rejection(
            "error::AuthError",
            format!("{}", other),
            StatusCode::UNAUTHORIZED,
        )
    }
}

pub fn routes(db: Arc<Db>) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    user::account_paths(db.clone())
        .recover(handle_rejection)
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    error!("ERROR - {:?}", err);

    let error_message = match err.find::<WebErrorMessage>() {
        Some(err) => err.clone(),
        None => WebErrorMessage::unknown()
    };
    info!("{}", error_message.message);

    let result = json!({
        "error": error_message.kind
    });
    let result = warp::reply::json(&result);

    Ok(warp::reply::with_status(result, error_message.status_code))
}

fn json_response<T: Serialize>(data: &T) -> Result<Json, Rejection> {
    let response = json!({
        "data": data
    });
    Ok(warp::reply::json(&response))
}

fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::header::headers_cloned()
        .and_then(auth)
}

async fn auth(auth_header: HeaderMap<HeaderValue>) -> Result<String, warp::Rejection> {
    let token = jwt_from_header(&auth_header)
        .ok_or(AuthorizationError::MissingAuthHeader)?;

    let token_data = match decode_jwt(&token) {
        Ok(data) => data,
        Err(err) => {
            match err {
                error::Error::JWTokenError(_) => return Err(AuthorizationError::from(err).into()),
                _ => return Err(err.into())
            }
        }
    };

    Ok(token_data.claims.sub())
}
