use std::sync::Arc;
use bson::oid::ObjectId;
use serde::Deserialize;
use serde_json::json;
use warp::{Filter, reject::Rejection, reply::Json};
use x3dh::handshake::{self};
use crate::{model::{Db, keys::{RegisterBundle, InitialMessage}, user::User}, server::with_auth, error::Error};
use super::json_response;

#[derive(Deserialize)]
struct AddInitialMessageBody {
    chat_id: ObjectId,
    message: handshake::InitialMessageBinary,
}

pub fn keys_paths(db: Arc<Db>) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let with_db = warp::any()
        .map(move || db.clone());
    let common = with_db.clone()
        .and(with_auth());

    let add_bundle = warp::path("bundle")
        .and(warp::path::end())
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(add_bundle_handle);

    let get_peer_bundle = warp::path("peer_bundle")
        .and(warp::path::end())
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(get_bundle_handle);

    let add_initial_message = warp::path("initial_message")
        .and(warp::path::end())
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(add_initial_message_handle);

    let get_initial_message = warp::path("get_initial_message")
        .and(warp::path::end())
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(get_initial_message_handle);

    add_bundle
        .or(get_peer_bundle)
        .or(add_initial_message)
        .or(get_initial_message)
}

async fn add_bundle_handle(db: Arc<Db>, oid: String, bundle: handshake::RegisterBundleBinary) -> Result<Json, Rejection> {
    let bundle = RegisterBundle::new(&oid, bundle)?;
    RegisterBundle::add_to_db(&db, &bundle).await?;

    let response = json!({
        "bundle": "ok"
    });
    json_response(&response)
}

async fn get_bundle_handle(db: Arc<Db>, oid: String, username: String) -> Result<Json, Rejection> {
    let user = User::get_by_username(&db, &username).await?;
    let user_id = user.id().ok_or(Error::InternalError)?;
    let bundle = RegisterBundle::get_by_user(&db, &user_id).await?.bundle.deserialize();
    let peer_bundle = handshake::PeerBundle {
        identity: bundle.identity,
        signature: bundle.signature,
        signed_pre: bundle.signed_pre,
        one_time_pre: bundle.one_time_pres.first().ok_or(Error::InternalError)?.clone(),
    }.serialize();
    let response = json!({
        "bundle": peer_bundle
    });
    json_response(&response)
}

async fn add_initial_message_handle(db: Arc<Db>, oid: String, body: AddInitialMessageBody) -> Result<Json, Rejection> {
    let message = InitialMessage::new(body.chat_id, body.message);
    InitialMessage::add_to_db(&db, &message).await?;
    let response = json!({
        "message": "ok"
    });
    json_response(&response)
}

async fn get_initial_message_handle(db: Arc<Db>, oid: String, chat_id: ObjectId) -> Result<Json, Rejection> {
    let message = InitialMessage::get_by_chat(&db, &chat_id).await?;
    let message = message.map(|m| m.message);
    let response = json!({
        "message": message
    });
    json_response(&response)
}
