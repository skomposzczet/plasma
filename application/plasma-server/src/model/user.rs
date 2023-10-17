use bson::doc;
use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
use crate::model::{Db, db, Error};
use crate::error;
use super::{objectid_from_str, from_document, BsonError};
use super::DATABASE;

const COLLECTION: &'static str  = "user";

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub username: String,
    password: String,
}

impl User {
    pub fn new(email: &String, username: &String, password: &String) -> User {
        User {
            id: None,
            email: email.clone(),
            username: username.clone(),
            password: password.clone(),
        }
    }

    pub fn id(self: &Self) -> Option<&ObjectId> {
        self.id.as_ref()
    }

    pub fn username(self: &Self) -> &String {
        &self.username
    }

    pub fn password_matches(self: &Self, hashed_password: &String) -> bool {
        self.password.eq(hashed_password)
    }

    pub async fn add_to_db(db: &Db, user: &User) -> Result<(), Error> {
        let bs = bson::to_bson(&user)
            .map_err(|err| BsonError::from(err))?;
        let document = bs.as_document()
            .ok_or(Error::BsonConvError(error::BsonError::ConversionError))?;

        let userdb = db
            .database(DATABASE)
            .collection::<mongodb::bson::Document>(COLLECTION);

        userdb.insert_one(document.to_owned(), None).await
            .map_err(|_| Error::DbError("insert", format!("{:?}", user)))?;

        Ok(())
    }

    pub async fn get_by_email(db: &Db, email: &String) -> Result<User, Error> {
        let filter = doc!{
            "email": email.as_str()
        };
        let document = db::get_by(db, &filter, &String::from("user"))
            .await?
            .ok_or(Error::NoUserWithSuchEmail)?;
        
        let user = from_document(document)?;

        Ok(user)
    }

    pub async fn get_by_id(db: &Db, id: &String) -> Result<User, Error> {
        let id = objectid_from_str(id)
            .map_err(|_| Error::InvalidOID)?;
        let filter = doc!{
            "_id": id
        };

        let document = db::get_by(db, &filter, &String::from("user"))
            .await?
            .ok_or(Error::DbError("find", id.to_string()))?;

        let user = from_document(document)?;
        
        Ok(user)
    }

    pub async fn get_by_username(db: &Db, username: &String) -> Result<User, Error> {
        let filter = doc!{
            "username": username.as_str()
        };
        let document = db::get_by(db, &filter, &String::from("user"))
            .await?
            .ok_or(Error::NoUserWithSuchEmail)?;
        
        let user = from_document(document)?;

        Ok(user)
    }
}
