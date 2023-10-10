#![allow(unused)]

mod model;
mod error;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("Connecting do db...");
    let db = Arc::new(model::db::init_db().await);
    println!("Successfully connected to db.");
}
