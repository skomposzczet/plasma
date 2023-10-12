use std::sync::Arc;
use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use serde_json::json;
use warp::{Filter, reject::Rejection, reply::Json};
use crate::model::{Db, message::Message, objectid_from_str};
use super::{with_auth, json_response};

#[derive(Deserialize)]
struct SendMessageBody {
    chat_id: ObjectId,
    message: String,
}

pub fn message_paths(db: Arc<Db>) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let with_db = warp::any()
        .map(move || db.clone());
    let common = with_db.clone()
        .and(with_auth());

    let get_messages = warp::path("messages")
        .and(warp::path::end())
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(get_messages_handle);

    let add_message = warp::path("message")
        .and(warp::path::end())
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(add_message_handle);

    get_messages
        .or(add_message)
}

async fn get_messages_handle(db: Arc<Db>, oid: String, chat_id: ObjectId) -> Result<Json, Rejection> {
    let messages = Message::get_messages_from_chat(&db, chat_id).await?;

    let response = json!({
        "messages": messages 
    });
    json_response(&response)
}

async fn add_message_handle(db: Arc<Db>, oid: String, body: SendMessageBody) -> Result<Json, Rejection> {
    let id = objectid_from_str(&oid).unwrap();
    let new_message = Message::new(body.chat_id, id, body.message);
    Message::add_to_db(&db, &new_message).await?;
    let response = json!({
        "send message": "ok"
    });
    json_response(&response)
}
