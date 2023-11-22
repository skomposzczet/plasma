use p256::ecdsa::VerifyingKey;
use p256::ecdsa::signature::Verifier;
use p256::elliptic_curve::ecdh::diffie_hellman;
use p256::{PublicKey, ecdsa::{SigningKey, signature::Signer}};
use rand::{CryptoRng, RngCore};
use sha2::digest::generic_array::GenericArray;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature (p256::ecdsa::Signature);

impl Signature {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let bytes = GenericArray::from_slice(bytes);
        Signature(p256::ecdsa::Signature::from_bytes(bytes).unwrap())
    }
}

pub trait Key {
    fn generate_for_private(private: &PrivateKey) -> Self;
    fn key(&self) -> &PublicKey;
    fn to_bytes(&self) -> Vec<u8> {
        self.key().to_sec1_bytes().to_vec()
    }
    fn from_bytes(bytes: &[u8]) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityKeyPublic (PublicKey);

impl IdentityKeyPublic {
    pub fn verify(&self, msg: &[u8], signature: &Signature) -> Result<(), p256::ecdsa::Error> {
        let vk = VerifyingKey::from(self.0);
        vk.verify(msg, &signature.0)
    }
}

impl Key for IdentityKeyPublic {
    fn generate_for_private(private: &PrivateKey) -> Self {
        IdentityKeyPublic(
            PublicKey::from_secret_scalar(&private.0.to_nonzero_scalar())
            )
    }

    fn key(&self) -> &PublicKey {
        &self.0
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        IdentityKeyPublic(PublicKey::from_sec1_bytes(bytes).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EphemeralKeyPublic (PublicKey);

impl Key for EphemeralKeyPublic {
    fn generate_for_private(private: &PrivateKey) -> Self {
        EphemeralKeyPublic(
            PublicKey::from_secret_scalar(&private.0.to_nonzero_scalar())
            )
    }

    fn key(&self) -> &PublicKey {
        &self.0
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        EphemeralKeyPublic(PublicKey::from_sec1_bytes(bytes).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedPreKeyPublic (PublicKey);

impl Key for SignedPreKeyPublic {
    fn generate_for_private(private: &PrivateKey) -> Self {
        SignedPreKeyPublic(
            PublicKey::from_secret_scalar(&private.0.to_nonzero_scalar())
            )
    }

    fn key(&self) -> &PublicKey {
        &self.0
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        SignedPreKeyPublic(PublicKey::from_sec1_bytes(bytes).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneTimePreKeyPublic (PublicKey);

impl Key for OneTimePreKeyPublic {
    fn generate_for_private(private: &PrivateKey) -> Self {
        OneTimePreKeyPublic(
            PublicKey::from_secret_scalar(&private.0.to_nonzero_scalar())
            )
    }

    fn key(&self) -> &PublicKey {
        &self.0
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        OneTimePreKeyPublic(PublicKey::from_sec1_bytes(bytes).unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateKey (p256::SecretKey);

impl PrivateKey {
    pub fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        PrivateKey(
            p256::SecretKey::random(rng)
        )
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let bytes = GenericArray::from_slice(bytes);
        PrivateKey( p256::SecretKey::from_bytes(bytes).unwrap() )
    }
}

pub struct SharedSecret (p256::ecdh::SharedSecret);

impl SharedSecret {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.raw_secret_bytes().to_vec()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct X3dhSharedSecret (Vec<u8>);

impl X3dhSharedSecret {
    pub fn to_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        X3dhSharedSecret(bytes.to_vec())
    }
}

pub trait KeyPair {
    type PairPublicKey: Key;

    fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self;
    fn public(&self) -> &Self::PairPublicKey;
    fn private(&self) -> &PrivateKey;
    fn diffie_hellman<K: Key>(&self, key: &K) -> SharedSecret {
        let sk = self.private().0.to_nonzero_scalar();
        let pk = key.key().as_affine();
        let dh = diffie_hellman(sk, pk);
        SharedSecret(dh)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let pk = self.public().to_bytes();
        let sk = self.private().to_bytes();
        bincode::serialize(&(pk, sk)).unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityKeyPair (IdentityKeyPublic, PrivateKey);

impl KeyPair for IdentityKeyPair {
    type PairPublicKey = IdentityKeyPublic;

    fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        let private = PrivateKey::generate(rng);
        IdentityKeyPair(
            IdentityKeyPublic::generate_for_private(&private),
            private,
        )
    }

    fn public(&self) -> &Self::PairPublicKey {
        &self.0
    }

    fn private(&self) -> &PrivateKey {
        &self.1
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let (pk, sk): (Vec<u8>, Vec<u8>) = bincode::deserialize(bytes).unwrap();
        IdentityKeyPair(Self::PairPublicKey::from_bytes(&pk), PrivateKey::from_bytes(&sk)) 
    }
}

impl IdentityKeyPair {
    pub fn sign(&self, msg: &[u8]) -> Signature {
        let sk = SigningKey::from(&self.1.0);
        Signature(sk.sign(msg))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EphemeralKeyPair (EphemeralKeyPublic, PrivateKey);

impl KeyPair for EphemeralKeyPair {
    type PairPublicKey = EphemeralKeyPublic;

    fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        let private = PrivateKey::generate(rng);
        EphemeralKeyPair(
            EphemeralKeyPublic::generate_for_private(&private),
            private,
        )
    }

    fn public(&self) -> &Self::PairPublicKey {
        &self.0
    }

    fn private(&self) -> &PrivateKey {
        &self.1
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let (pk, sk): (Vec<u8>, Vec<u8>) = bincode::deserialize(bytes).unwrap();
        EphemeralKeyPair(Self::PairPublicKey::from_bytes(&pk), PrivateKey::from_bytes(&sk)) 
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedPreKeyPair (SignedPreKeyPublic, PrivateKey);

impl KeyPair for SignedPreKeyPair {
    type PairPublicKey = SignedPreKeyPublic ;
    
    fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        let private = PrivateKey::generate(rng);
        SignedPreKeyPair(
            SignedPreKeyPublic::generate_for_private(&private),
            private,
        )
    }

    fn public(&self) -> &Self::PairPublicKey{
        &self.0
    }

    fn private(&self) -> &PrivateKey {
        &self.1
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let (pk, sk): (Vec<u8>, Vec<u8>) = bincode::deserialize(bytes).unwrap();
        SignedPreKeyPair(Self::PairPublicKey::from_bytes(&pk), PrivateKey::from_bytes(&sk)) 
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneTimeKeyPair (OneTimePreKeyPublic, PrivateKey, u16);

impl KeyPair for OneTimeKeyPair {
    type PairPublicKey = OneTimePreKeyPublic ;

    fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        let private = PrivateKey::generate(rng);
        OneTimeKeyPair(
            OneTimePreKeyPublic::generate_for_private(&private),
            private,
            0u16,
        )
    }

    fn public(&self) -> &Self::PairPublicKey {
        &self.0
    }

    fn private(&self) -> &PrivateKey {
        &self.1
    }

    fn to_bytes(&self) -> Vec<u8> {
        let pk = self.public().to_bytes();
        let sk = self.private().to_bytes();
        let idx = self.index();
        bincode::serialize(&(pk, sk, idx)).unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let (pk, sk, idx): (Vec<u8>, Vec<u8>, u16) = bincode::deserialize(bytes).unwrap();
        OneTimeKeyPair(Self::PairPublicKey::from_bytes(&pk), PrivateKey::from_bytes(&sk), idx) 
    }
}

impl OneTimeKeyPair {
    pub fn with_index(mut self, index: u16) -> Self {
        self.2 = index;
        self
    }

    pub fn index(&self) -> u16 {
        self.2
    }
}

#[cfg(test)]
mod keys_test {
    use super::{IdentityKeyPair, KeyPair, SignedPreKeyPair, Key};

    fn random_identity_key() -> IdentityKeyPair {
        let mut rng = rand::rngs::OsRng::default();
        IdentityKeyPair::generate(&mut rng)
    }

    #[test]
    fn identity_dh() {
        let id1 = random_identity_key();
        let id2 = random_identity_key();
        
        let dh1 = id1.diffie_hellman(id2.public());
        let dh2 = id2.diffie_hellman(id1.public());

        assert_eq!(dh1.to_bytes(), dh2.to_bytes());
    }

    #[test]
    fn idenity_dh_wrong_key() {
        let id1 = random_identity_key();
        let id2 = random_identity_key();
        let id3 = random_identity_key();

        let dh1 = id1.diffie_hellman(id2.public());
        let dh2 = id2.diffie_hellman(id3.public());

        assert_ne!(dh1.to_bytes(), dh2.to_bytes());
    }

    #[test]
    fn identity_sign_verify() {
        let identity = random_identity_key();
        let mut rng = rand::rngs::OsRng::default();
        let signed = SignedPreKeyPair::generate(&mut rng);

        let signature = identity.sign(&signed.public().to_bytes());
        let result = identity
            .public()
            .verify(&signed.public().to_bytes(), &signature);

        assert!(result.is_ok());
    }

    #[test]
    fn identity_sign_verify_wrong_signed() {
        let identity = random_identity_key();
        let mut rng = rand::rngs::OsRng::default();
        let signed1 = SignedPreKeyPair::generate(&mut rng);
        let signed2 = SignedPreKeyPair::generate(&mut rng);

        let signature = identity.sign(&signed1.public().to_bytes());
        let result = identity
            .public()
            .verify(&signed2.public().to_bytes(), &signature);

        assert!(result.is_err());
    }

    #[test]
    fn identity_sign_verify_wrong_identity() {
        let identity1 = random_identity_key();
        let identity2 = random_identity_key();
        let mut rng = rand::rngs::OsRng::default();
        let signed = SignedPreKeyPair::generate(&mut rng);

        let signature = identity1.sign(&signed.public().to_bytes());
        let result = identity2
            .public()
            .verify(&signed.public().to_bytes(), &signature);

        assert!(result.is_err());
    }

    #[test]
    fn identity_sign_different_keys_different_signatures() {
        let identity1 = random_identity_key();
        let identity2 = random_identity_key();
        let mut rng = rand::rngs::OsRng::default();
        let signed = SignedPreKeyPair::generate(&mut rng);

        let signature1 = identity1.sign(&signed.public().to_bytes());
        let signature2 = identity2.sign(&signed.public().to_bytes());

        assert_ne!(signature1, signature2);
    }

    #[test]
    fn identity_same_from_bytes_same() {
        let identity1 = random_identity_key();
        let identity2 = IdentityKeyPair::from_bytes(&identity1.to_bytes());

        assert_eq!(identity1, identity2);
    }
}
