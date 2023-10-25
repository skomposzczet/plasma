pub mod clients;

use std::{sync::Arc, str::FromStr};
use bson::oid::ObjectId;
use futures::{StreamExt, SinkExt, TryFutureExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use warp::{Filter, reject::Rejection, reply::Reply, ws::WebSocket};
use crate::{model::{Db, chat::Chat, message::Message}, server::with_auth, ClientsHandle, error};
use tokio_stream::wrappers::UnboundedReceiverStream;

#[derive(Serialize, Deserialize)]
struct WsMessage {
    chat_id: String,
    sender_id: String,
    content: String,
}

pub fn ws_paths(db: Arc<Db>, clients: ClientsHandle) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
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

async fn handle(ws: warp::ws::Ws, db: Arc<Db>, clients: ClientsHandle, oid: String) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(move |socket| user_connected(socket, db.clone(), clients.clone(), oid)))
}

async fn user_connected(socket: WebSocket, db: Arc<Db>, clients: ClientsHandle, oid: String) {
    debug!("User connected: {}", oid);

    let (mut user_ws_tx, mut user_ws_rx) = socket.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    clients.write().await.add_client(oid.clone(), tx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    error!("websocket err: {}", e);
                })
                .await;
        }
    });

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("websocket error: id={}, error={}", oid, e);
                break;
            },
        };
        user_message(db.clone(), &oid, msg, clients.clone()).await;
    }
    disconnect_user(&oid, clients.clone()).await;
}

async fn user_message(db: Arc<Db>, oid: &str, msg: warp::filters::ws::Message, clients: ClientsHandle) {
    if msg.as_bytes().is_empty() {
        return;
    }
    let ws_msg: WsMessage = bincode::deserialize(msg.as_bytes()).unwrap();
    let chat = Chat::get_by_id(&db, &ws_msg.chat_id).await.unwrap();
    let sender_id = ObjectId::from_str(&ws_msg.sender_id).unwrap();

    let new_message = Message::new(chat.id().unwrap(), sender_id, ws_msg.content);
    if let Err(e) = Message::add_to_db(&db, &new_message).await {
        error!("Failed to send message: {}", e);
        return;
    }

    let members = chat.members()
        .iter()
        .filter(|&m| *m != sender_id);

    for member in members {
        let member_id_str = format!("ObjectId(\"{}\")", member.to_string());
        match clients.write().await.get_client(&member_id_str) {
            None => {
                info!("No client for {}", member);
                return
            },
            Some(client) => {
                let res = client.send(msg.clone());
                if res.is_err() {
                    error!("User disconnected {}", oid);
                    return;
                }
            }
        }
    }
}

async fn disconnect_user(oid: &str, clients: ClientsHandle) {
    info!("User disconnected: {}", oid);
    clients.write().await.remove_client(oid);
}
