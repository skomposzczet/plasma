extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod model;
mod server;
mod rest;
mod ws;
mod security;
mod error;

use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;

type ClientsHandle = Arc<RwLock<ws::clients::Clients>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("Connecting do db...");
    let db = Arc::new(model::db::init_db().await);
    info!("Successfully connected to db.");

    let clients = Arc::new(RwLock::new(ws::clients::Clients::new()));

    let cors = warp::cors()
        .allow_any_origin();

    let log = warp::log("server::plasma");

    let routes = server::routes(db.clone(), clients.clone())
        .with(cors)
        .with(log);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8000)).await;
}
