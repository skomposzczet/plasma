use mongodb::{Client, options::{ClientOptions, FindOptions}};
use dotenv;
use bson::{doc, Document};
use super::{DATABASE, Error};
use futures::TryStreamExt;

const MONGO_USER: &str = "MONGO_USER";
const MONGO_PW: &str = "MONGO_PW";
const MONGO_HOST: &str = "MONGO_HOST";
const MONGO_PORT: &str = "MONGO_PORT";

pub type Db = Client;

pub async fn init_db() -> Db {
    let client_uri = make_client_uri().unwrap();
    let options = ClientOptions::parse(&client_uri).await.unwrap();
    let client = Client::with_options(options).unwrap();
    check_db_conn(&client).await.unwrap();

    client
}

fn make_client_uri() -> Result<String, Box<dyn std::error::Error>> {
    dotenv::dotenv()?;

    let user = dotenv::var(MONGO_USER)?;
    let password = dotenv::var(MONGO_PW)?;
    let host = dotenv::var(MONGO_HOST)?;
    let port = dotenv::var(MONGO_PORT)?;

    Ok(format!("mongodb://{}:{}@{}:{}",
        user, password, host, port))
}

async fn check_db_conn(db: &Db) -> Result<(), Error> {
    db.database(DATABASE)
        .run_command(doc! {"ping": 1}, None)
        .await
        .map_err(|_| Error::CouldNotConnectToDB)?;
    
    Ok(())
}

pub async fn get_by(db: &Db, filter: &Document, collection: &String) -> Result<Option<Document>, Error> {
    let db = db
        .database(DATABASE)
        .collection::<mongodb::bson::Document>(collection);

    let document = db.find_one(filter.clone(), None)
        .await
        .map_err(|_| Error::DbError("find", filter.to_string()))?;
    
    Ok(document)
}

pub async fn get_all_in_vec(db: &Db, filter: Document, options: impl Into<Option<FindOptions>>, collection: &str) -> Result<Vec<Document>, Error> {
    let db = db
        .database(DATABASE)
        .collection::<mongodb::bson::Document>(collection);

    let cursor = db.find(filter.clone(), options).await
        .map_err(|_| Error::DbError("find", filter.to_string()))?;

    let results: Vec<Document> = cursor.try_collect().await
        .map_err(|_| Error::DbError("find", filter.to_string()))?;

    Ok(results)
}

