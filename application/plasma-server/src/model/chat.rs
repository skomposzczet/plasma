use std::sync::Arc;

use bson::doc;
use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use crate::model::{Db, db, Error};
use crate::error;
use super::{objectid_from_str, from_document, BsonError};
use super::DATABASE;

const COLLECTION: &'static str  = "chat";

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    users: Vec<ObjectId>,
}

impl Chat {
    pub fn new(user1: &str, user2: ObjectId) -> Result<Self, Error> {
        let id = objectid_from_str(user1)
            .map_err(|_| Error::InvalidOID)?;
        let chat = Chat {
            id: None,
            users: vec!(id, user2),
        };
        Ok(chat)
    }

    pub fn id(&self) -> &Option<ObjectId> {
        &self.id
    }

    pub async fn add_to_db(db: &Db, chat: &Chat) -> Result<ObjectId, Error> {
        let bs = bson::to_bson(&chat)
            .map_err(|err| BsonError::from(err))?;
        let document = bs.as_document()
            .ok_or(Error::BsonConvError(error::BsonError::ConversionError))?;

        let result = db.database(DATABASE)
            .collection::<mongodb::bson::Document>(COLLECTION)
            .insert_one(document.to_owned(), None).await
            .map_err(|_| Error::DbError("insert", format!("{:?}", chat)))?;
        let id = result.inserted_id.as_object_id().unwrap();

        Ok(id)
    }

    pub async fn get_by_users(db: &Db, id1: &str, id2: &ObjectId) -> Result<Chat, Error> {
        let id1 = objectid_from_str(id1)
            .map_err(|_| Error::InvalidOID)?;
        let filter = doc!{
            "$and": [{
                "users": {
                    "$elemMatch": {
                        "$eq": id1
                    }
                }
            },{
                "users": {
                    "$elemMatch": {
                        "$eq": id2
                    }
                }
            }]
        };

        let document = db::get_by(db, &filter, &String::from("chat"))
            .await?
            .ok_or(Error::DbError("find", id1.to_string()))?;

        let chat = from_document(document)?;
        
        Ok(chat)
    }

    pub async fn get_users_chats(db: Arc<Db>, id: &str) -> Result<Vec<Chat>, Error> {
        let id = objectid_from_str(id)
            .map_err(|_| Error::InvalidOID)?;
        let filter = doc!{
            "users": {
                "$elemMatch": {
                    "$eq": id
                }
            }
        };
        
        let documents = db::get_all_in_vec(&db, filter, None, COLLECTION).await?;
        let mut chats: Vec<Chat> = vec![];
        for doc in documents {
            let list = from_document(doc.clone())?;
            chats.push(list);
        }

        Ok(chats)
    }
}
