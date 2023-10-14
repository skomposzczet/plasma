pub mod clients;

use std::sync::Arc;
use futures::{StreamExt, SinkExt, TryFutureExt};
use tokio::sync::mpsc;
use warp::{Filter, reject::Rejection, reply::Reply, ws::WebSocket};
use crate::{model::Db, server::with_auth, ClientsHandle};
use tokio_stream::wrappers::UnboundedReceiverStream;

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

async fn user_connected(mut socket: WebSocket, db: Arc<Db>, clients: ClientsHandle, oid: String) {
    debug!("User connected: {}", oid);

    for i in 0..5 {
        if socket.send(warp::filters::ws::Message::text(format!("hello {}", i))).await.is_err() {
            error!(":< dc {}", oid);
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    }

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
        user_message(&oid, msg, clients.clone()).await;
    }
    disconnect_user(&oid, clients.clone()).await;
}

async fn user_message(oid: &str, msg: warp::filters::ws::Message, clients: ClientsHandle) {
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        error!("non text msg");
        return;
    };
    let msg = format!("<{}>: {}", oid, msg);

    match clients.write().await.get_client(oid) {
        None => {
            error!("no client for {}", oid);
            return
        },
        Some(client) => {
            let res = client.send(warp::filters::ws::Message::text(msg));
            if res.is_err() {
                error!("User disconnected {}", oid);
                return;
            }
        }
    }
}

async fn disconnect_user(oid: &str, clients: ClientsHandle) {
    debug!("User disconnected: {}", oid);
    clients.write().await.remove_client(oid);
}
