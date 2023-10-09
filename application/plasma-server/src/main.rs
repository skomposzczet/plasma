use std::sync::Arc;

#[tokio::main]
async fn main() -> Option(()) {
    println!("Connecting do db...");
    let db = Arc::new(model::db::init_db().await);
    println!("Successfully connected to db.");
    None
}
