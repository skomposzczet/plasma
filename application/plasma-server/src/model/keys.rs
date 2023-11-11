use bson::{oid::ObjectId, doc};
use serde::{Serialize, Deserialize};
use x3dh::handshake;
use crate::error::BsonError;
use crate::error;
use super::{Error, Db, DATABASE, db, from_document, objectid_from_str};

const BUNDLE_COLLECTION: &'static str = "bundle";
const INITIAL_MESSAGE_COLLECTION: &'static str = "initial_message";

#[derive(Serialize, Deserialize)]
pub struct RegisterBundle {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub bundle: handshake::RegisterBundleBinary,
}

#[derive(Serialize, Deserialize)]
pub struct InitialMessage {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub chat_id: ObjectId,
    pub message: handshake::InitialMessageBinary,
}

impl RegisterBundle {
    pub fn new(user_id: &str, bundle: handshake::RegisterBundleBinary) -> Result<RegisterBundle, Error> {
        let user_id = objectid_from_str(user_id)
            .map_err(|_| Error::InvalidOID)?;
        let rb = RegisterBundle {
            id: None,
            user_id,
            bundle,
        };
        Ok(rb)
    }

    pub async fn add_to_db(db: &Db, bundle: &RegisterBundle) -> Result<(), Error> {
        let bs = bson::to_bson(bundle)
            .map_err(|err| BsonError::from(err))?;
        let document = bs.as_document()
            .ok_or(Error::BsonConvError(error::BsonError::ConversionError))?;

        let bundledb = db
            .database(DATABASE)
            .collection::<mongodb::bson::Document>(BUNDLE_COLLECTION);

        bundledb.insert_one(document.to_owned(), None).await
            .map_err(|_| Error::DbError("insert bundle", format!("{:?}", bundle.user_id)))?;

        Ok(())
    }

    pub async fn get_by_user(db: &Db, user_id: &ObjectId) -> Result<RegisterBundle, Error> {
        let filter = doc!{
            "user_id": user_id
        };

        let document = db::get_by(db, &filter, &String::from(BUNDLE_COLLECTION))
            .await?
            .ok_or(Error::DbError("get bundle", user_id.to_string()))?;

        let bundle = from_document(document)?;
        
        Ok(bundle)
    }
}

impl InitialMessage {
    pub fn new(chat_id: ObjectId, message: handshake::InitialMessageBinary) -> InitialMessage {
        InitialMessage {
            id: None,
            chat_id,
            message,
        }
    }

    pub async fn add_to_db(db: &Db, message: &InitialMessage) -> Result<(), Error> {
        let bs = bson::to_bson(message)
            .map_err(|err| BsonError::from(err))?;
        let document = bs.as_document()
            .ok_or(Error::BsonConvError(error::BsonError::ConversionError))?;

        let messagedb = db
            .database(DATABASE)
            .collection::<mongodb::bson::Document>(INITIAL_MESSAGE_COLLECTION);

        messagedb.insert_one(document.to_owned(), None).await
            .map_err(|_| Error::DbError("insert message", format!("{:?}", message.chat_id)))?;

        Ok(())
    }

    pub async fn get_by_chat(db: &Db, chat_id: &ObjectId) -> Result<Option<InitialMessage>, Error> {
        let filter = doc!{
            "chat_id": chat_id
        };

        let document = db::get_by(db, &filter, &String::from(INITIAL_MESSAGE_COLLECTION))
            .await?;

        match document {
            Some(doc) => Ok(from_document(doc)?),
            None => Ok(None),
        }
    }
}
