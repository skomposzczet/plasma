use bson::document::ValueAccessError;
use thiserror::Error;

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
