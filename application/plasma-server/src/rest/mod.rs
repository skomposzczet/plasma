mod user;
mod chat;
mod message;

use std::sync::Arc;
use serde::Serialize;
use serde_json::json;
use warp::{Filter, reply::{Reply, Json}, reject::Rejection};
use crate::model::Db;

pub fn rest_routes(db: Arc<Db>) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    user::account_paths(db.clone())
        .or(chat::chat_paths(db.clone()))
        .or(message::message_paths(db.clone()))

}

fn json_response<T: Serialize>(data: &T) -> Result<Json, Rejection> {
    let response = json!({
        "data": data
    });
    Ok(warp::reply::json(&response))
}

