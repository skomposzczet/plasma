use std::{sync::Arc, collections::HashMap};
use bson::oid::ObjectId;
use serde_json::json;
use warp::{Filter, reject::Rejection, reply::Json};
use crate::{model::{Db, chat::Chat, user::User}, error::Error};
use super::{with_auth, json_response};

pub fn chat_paths(db: Arc<Db>) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let with_db = warp::any()
        .map(move || db.clone());
    let common = with_db.clone()
        .and(with_auth());

    let get_chats = warp::path("chats")
        .and(warp::path::end())
        .and(warp::get())
        .and(common.clone())
        .and_then(get_chats_handle);

    let add_chat = warp::path("chat")
        .and(warp::path::end())
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(add_chat_handle);

    get_chats
        .or(add_chat)
}

async fn get_chats_handle(db: Arc<Db>, oid: String) -> Result<Json, Rejection> {
    let chats = Chat::get_users_chats(db, &oid).await?;

    let response = json!({
        "chats": chats 
    });
    json_response(&response)
}

async fn add_chat_handle(db: Arc<Db>, oid: String, body: HashMap<String, String>) -> Result<Json, Rejection> {
    let member_name = body.get("member")
        .ok_or(Error::BodyError("member"))?;
    let member = User::get_by_username(&db, &member_name).await?;
    let member_id = member.id().ok_or(Error::InternalError)?;
    let id = match Chat::get_by_users(&db, &oid, &member_id).await {
        Ok(chat) => {
            chat.id().to_owned()
        },
        Err(_) => {
            let chat = Chat::new(&oid, member_id.to_owned())?;
            Chat::add_to_db(&db, &chat).await.ok()
        },
    };
    let id = id.ok_or(Error::InternalError)?;

    let response = json!({
        "Chat id": id
    });
    json_response(&response)
}
