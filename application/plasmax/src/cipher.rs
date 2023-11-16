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

#[cfg(test)]
mod cipher_tests {
    use rand::Rng;
    use x3dh::keys::X3dhSharedSecret;
    use super::Cipher;

    fn cipher_random_key() -> Cipher {
        let bytes = rand::thread_rng().gen::<[u8; 32]>();
        Cipher::new(X3dhSharedSecret::from_bytes(&bytes))
    }

    #[test]
    fn enc_dec_correct() {
        let cipher = cipher_random_key();
        let message = String::from("message");
        let timestamp = 0u64;

        let encrypted = cipher.encrypt(&message, timestamp)
            .unwrap();
        let decrypted = cipher.decrypt(&encrypted, timestamp)
            .unwrap();

        assert_eq!(message, decrypted);
    }

    #[test]
    fn enc_dec_different_timestamp() {
        let cipher = cipher_random_key();
        let message = String::from("message");
        let timestamp1 = 0u64;
        let timestamp2 = 1u64;

        let encrypted = cipher.encrypt(&message, timestamp1)
            .unwrap();
        let decrypted_result = cipher.decrypt(&encrypted, timestamp2);

        assert!(decrypted_result.is_err());
    }

    #[test]
    fn enc_dec_different_secret_key() {
        let cipher1 = cipher_random_key();
        let cipher2 = cipher_random_key();
        let message = String::from("message");
        let timestamp = 0u64;

        let encrypted = cipher1.encrypt(&message, timestamp)
            .unwrap();
        let decrypted_result = cipher2.decrypt(&encrypted, timestamp);

        assert!(decrypted_result.is_err());
    }
}
