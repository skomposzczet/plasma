use std::{sync::Arc, convert::Infallible};
use serde::Serialize;
use serde_json::json;
use crate::{model::{Db, self}, error};
use warp::{reply::Json, Rejection, Filter, hyper::{HeaderMap, StatusCode}, http::HeaderValue, Reply};

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

pub fn routes(db: Arc<Db>) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    warp::path("hello")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| {
            format!("Hello {}, whose agent is {}", param, agent)
        })
        .recover(handle_rejection)
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let error_message = match err.find::<WebErrorMessage>() {
        Some(err) => err.clone(),
        None => WebErrorMessage::unknown()
    };

    let result = json!({
        "error": error_message.kind
    });
    let result = warp::reply::json(&result);

    Ok(warp::reply::with_status(result, error_message.status_code))
}

