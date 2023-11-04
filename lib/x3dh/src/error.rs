#[derive(thiserror::Error, Debug)]
pub enum X3dhError {
    #[error("Validation of signature failed")]
    ValidationError(#[from] p256::ecdsa::Error),
}
