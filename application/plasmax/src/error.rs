use thiserror::Error;
use x3dh::error::X3dhError;
use crate::{api::ApiError, cipher::CipherError};

#[derive(Error, Debug)]
pub enum PlasmaError {
    #[error(transparent)]
    IoError( #[from] std::io::Error ),
    #[error(transparent)]
    ServerError( #[from] ApiError),
    #[error(transparent)]
    MessageCipherError( #[from] CipherError),
    #[error(transparent)]
    X3dhLibError( #[from] X3dhError),
}
