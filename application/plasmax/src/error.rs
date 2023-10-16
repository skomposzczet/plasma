use thiserror::Error;
use crate::api::ApiError;

#[derive(Error, Debug)]
pub enum PlasmaError {
    #[error(transparent)]
    IoError( #[from] std::io::Error ),
    #[error(transparent)]
    ServerError( #[from] ApiError),
}
