pub mod clients;


use std::{sync::Arc, collections::HashMap};
use bson::oid::ObjectId;
use serde_json::json;
use warp::{Filter, reject::Rejection, reply::Json};
use crate::{model::{Db, chat::Chat, user::User}, error::Error, rest::with_auth};

pub fn ws_paths(db: Arc<Db>) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let with_db = warp::any()
        .map(move || db.clone());
    let common = with_db.clone()
        .and(with_auth());

    let chat = warp::path("chat")
        .and(warp::ws())
        .and(common.clone())
        .and_then(handle);

    chat
}

async fn handle(ws: warp::ws::Ws, db: Arc<Db>, oid: String) -> Result<Json, Rejection> {
    todo!()
}

