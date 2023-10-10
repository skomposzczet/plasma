use bson::document::ValueAccessError;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    JWTokenError( #[from] jsonwebtoken::errors::Error ),
    #[error(transparent)]
    EnvError( #[from] dotenv::Error ),
    #[error("Expected key: {0}")]
    BodyError(&'static str),
    #[error("Invalid claim data: {0}")]
    InvalidClaimData(&'static str),
}

#[derive(Error, Debug)]
pub enum BsonError {
    #[error(transparent)]
    ValueAccess (#[from] ValueAccessError),
    #[error(transparent)]
    BsonValueAccess (#[from] bson::raw::ValueAccessError),
    #[error(transparent)]
    BsonError (#[from]bson::ser::Error),
    #[error("Conversion error")]
    ConversionError,
}

#[derive(Error, Debug)]
pub enum AuthorizationError {
    #[error(transparent)]
    InvalidToken( #[from] Error ),
    #[error("Invalid credentials: {0}")]
    InvalidCredentials(&'static str),
    #[error("Missing authorization header")]
    MissingAuthHeader,
}

