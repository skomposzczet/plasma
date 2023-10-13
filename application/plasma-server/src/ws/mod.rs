pub mod clients;

use std::sync::Arc;
use warp::{Filter, reject::Rejection, reply::Json};
use crate::{model::Db, server::with_auth, ClientsHandle};

pub fn ws_paths(db: Arc<Db>, clients: ClientsHandle) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let with_db = warp::any()
        .map(move || db.clone());
    let with_clients = warp::any()
        .map(move || clients.clone());
    let common = with_db.clone()
        .and(with_clients.clone())
        .and(with_auth());

    let chat = warp::path("chat")
        .and(warp::ws())
        .and(common.clone())
        .and_then(handle);

    chat
}

async fn handle(ws: warp::ws::Ws, db: Arc<Db>, clients: ClientsHandle, oid: String) -> Result<Json, Rejection> {
    todo!()
}

