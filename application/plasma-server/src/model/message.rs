use bson::doc;
use bson::oid::ObjectId;
use mongodb::options::FindOptions;
use futures::TryStreamExt;
use serde::{Serialize, Deserialize};
use crate::model::{Db, db, Error};
use crate::error;
use super::{objectid_from_str, from_document, BsonError};
use super::DATABASE;

const COLLECTION: &'static str  = "message";

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    chat_id: ObjectId,
    sender_id: ObjectId,
    message: String,
}

impl Message {
    pub fn new(chat_id: ObjectId, sender_id: ObjectId, message: String) -> Self {
        Message {
            id: None,
            chat_id,
            sender_id,
            message,
        }
    }

    pub async fn add_to_db(db: &Db, message: &Message) -> Result<(), Error> {
        let bs = bson::to_bson(&message)
            .map_err(|err| BsonError::from(err))?;
        let document = bs.as_document()
            .ok_or(Error::BsonConvError(error::BsonError::ConversionError))?;

        db.database(DATABASE)
            .collection::<mongodb::bson::Document>(COLLECTION)
            .insert_one(document.to_owned(), None).await
            .map_err(|_| Error::DbError("insert", format!("{:?}", message)))?;

        Ok(())
    }

    pub async fn get_messages_from_chat(db: &Db, chat_id: ObjectId) -> Result<Vec<Message>, Error> {
        let filter = doc!{
            "chat_id": chat_id
        };
        let options = FindOptions::builder()
            .sort(doc!{"_id": 1})
            .build();

        let db = db
            .database(DATABASE)
            .collection::<Message>(COLLECTION);
        
        let cursor = db.find(filter.clone(), options).await
            .map_err(|_| Error::DbError("find", filter.to_string()))?;
        let results: Vec<Message> = cursor.try_collect().await
            .map_err(|_| Error::DbError("find", filter.to_string()))?;

        Ok(results)
    }
}
