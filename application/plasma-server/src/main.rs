extern crate pretty_env_logger;
#[macro_use] extern crate log;

mod model;
mod server;
mod rest;
mod ws;
mod security;
mod error;

use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    println!("Connecting do db...");
    let db = Arc::new(model::db::init_db().await);
    println!("Successfully connected to db.");

    let cors = warp::cors()
        .allow_any_origin();

    let log = warp::log("server::plasma");

    let routes = server::routes(db.clone())
        .with(cors)
        .with(log);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8000)).await;
}
