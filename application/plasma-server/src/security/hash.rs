use sha3::{Digest, Sha3_256};

pub fn hashed_password(password: &String) -> String {
    format!("{:x}", Sha3_256::digest(password.as_bytes()))
}
