use warp::hyper::StatusCode;
use crate::model;
use crate::error;

#[derive(Debug, Clone)]
pub struct WebErrorMessage {
    pub kind: &'static str,
    pub message: String,
    pub status_code: StatusCode,
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

