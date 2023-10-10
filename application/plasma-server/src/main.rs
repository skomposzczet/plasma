#![allow(unused)]

mod model;
mod rest;
mod security;
mod error;

use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() {
    println!("Connecting do db...");
    let db = Arc::new(model::db::init_db().await);
    println!("Successfully connected to db.");

    let cors = warp::cors()
        .allow_any_origin();

    let log = warp::log("server::plasma");

    let routes = rest::routes(db.clone())
        .with(cors)
        .with(log);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8000)).await;
}
