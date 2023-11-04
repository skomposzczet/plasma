use p256::ecdsa::VerifyingKey;
use p256::ecdsa::signature::Verifier;
use p256::elliptic_curve::ecdh::diffie_hellman;
use p256::{PublicKey, ecdsa::{SigningKey, signature::Signer}};
use rand::{CryptoRng, RngCore};

pub struct Signature (p256::ecdsa::Signature);

pub trait Key {
    fn generate_for_private(private: &PrivateKey) -> Self;
    fn key(&self) -> &PublicKey;
}

#[derive(Clone)]
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
}

#[derive(Clone)]
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
}

#[derive(Clone)]
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
}

#[derive(Clone)]
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
}

pub struct PrivateKey (p256::SecretKey);

impl PrivateKey {
    pub fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        PrivateKey(
            p256::SecretKey::random(rng)
        )
    }
}

pub struct SharedSecret (p256::ecdh::SharedSecret);

impl SharedSecret {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.raw_secret_bytes().to_vec()
    }
}

pub struct X3dhSharedSecret (Vec<u8>);

impl X3dhSharedSecret {
    pub fn to_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl X3dhSharedSecret {
    pub fn from(bytes: &[u8]) -> Self {
        X3dhSharedSecret(bytes.to_vec())
    }
}

pub trait KeyPair {
    type PairPublicKey;

    fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self;
    fn public(&self) -> &Self::PairPublicKey;
    fn private(&self) -> &PrivateKey;
    fn diffie_hellman<K: Key>(&self, key: &K) -> SharedSecret {
        let sk = self.private().0.to_nonzero_scalar();
        let pk = key.key().as_affine();
        let dh = diffie_hellman(sk, pk);
        SharedSecret(dh)
    }
}

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

}

impl IdentityKeyPair {
    pub fn sign(&self, msg: &[u8]) -> Signature {
        let sk = SigningKey::from(&self.1.0);
        Signature(sk.sign(msg))
    }
}

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
}

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
}

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
