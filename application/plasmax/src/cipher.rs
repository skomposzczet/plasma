use std::string::FromUtf8Error;
use x3dh::keys::X3dhSharedSecret;
use chacha20poly1305::{
    aead::{Aead, KeyInit, generic_array::GenericArray},
    ChaCha20Poly1305
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CipherError {
    #[error("Encryption error: {0}")]
    EncryptionError(chacha20poly1305::aead::Error),
    #[error("Decryption error: {0}")]
    DecryptionError(chacha20poly1305::aead::Error),
    #[error(transparent)]
    ConversionError(#[from] FromUtf8Error),
}

pub struct Cipher {
    secret: X3dhSharedSecret,
}

impl Cipher {
    pub fn new(secret: X3dhSharedSecret) -> Self {
        Cipher {
            secret,
        }
    }

    pub fn encrypt(&self, message: &str, timestamp: u64) -> Result<Vec<u8>, CipherError> {
        let secret = self.secret.to_bytes();
        let secret = GenericArray::from_slice(&secret);
        let mut timestamp = timestamp.to_le_bytes().to_vec();
        timestamp.resize(12, 0);

        let cipher = ChaCha20Poly1305::new(&secret);
        let nonce = GenericArray::from_slice(&timestamp[0..12]);

        cipher
            .encrypt(&nonce, message.as_bytes())
            .map_err(|e| CipherError::EncryptionError(e))
    }

    pub fn decrypt(&self, message_bytes: &[u8], timestamp: u64) -> Result<String, CipherError> {
        let secret = self.secret.to_bytes();
        let secret = GenericArray::from_slice(&secret);
        let mut timestamp = timestamp.to_le_bytes().to_vec();
        timestamp.resize(12, 0);

        let cipher = ChaCha20Poly1305::new(&secret);
        let nonce = GenericArray::from_slice(&timestamp[0..12]);

        let message = cipher.decrypt(&nonce, message_bytes)
            .map_err(|e| CipherError::DecryptionError(e))?;
        let message = String::from_utf8(message)?;
        Ok(message)
    }
}
