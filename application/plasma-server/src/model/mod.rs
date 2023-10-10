pub mod db;
pub mod user;

pub use db::Db;

use serde::de::DeserializeOwned;
use std::str::FromStr;
use bson::{oid::ObjectId, Document, Bson};
use thiserror::Error;
use crate::error::BsonError;

const DATABASE: &'static str = "plasma";

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    BsonConvError (#[from] BsonError),
    #[error("Couldn't {0} DB: {1}")]
    DbError(&'static str, String),
    #[error("No user with such email")]
    NoUserWithSuchEmail,
    #[error("Failed connecting to db")]
    CouldNotConnectToDB,
    #[error("Invalid ObjectId")]
    InvalidOID,
    #[error("Already in use: {0}")]
    NotUnique(&'static str)
}

pub fn objectid_from_str(id: &str) -> Result<ObjectId, Error> {
    objectid_from_str_raw(&id[10..34])
}

pub fn objectid_from_str_raw(id: &str) -> Result<ObjectId, Error> {
    mongodb::bson::oid::ObjectId::from_str(&id)
        .map_err(|_| Error::InvalidOID)
}

pub fn from_document<T: DeserializeOwned>(document: Document) -> Result<T, BsonError> {
    bson::from_bson(Bson::Document(document))
        .map_err(|_| BsonError::ConversionError)
}

