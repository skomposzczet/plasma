use p256::elliptic_curve::ecdh::diffie_hellman;
use p256::{PublicKey, ecdsa::{SigningKey, signature::Signer}};
use rand::{CryptoRng, RngCore};

pub struct Signature (p256::ecdsa::Signature);

pub trait Key {
    fn key(&self) -> &PublicKey;
}

#[derive(Clone)]
pub struct IdentityKeyPublic (PublicKey);

impl IdentityKeyPublic {
    pub fn generate_for_private(private: &PrivateKey) -> Self {
        IdentityKeyPublic(
            PublicKey::from_secret_scalar(&private.0.to_nonzero_scalar())
        )
    }
}

impl Key for IdentityKeyPublic {
    fn key(&self) -> &PublicKey {
        &self.0
    }
}

#[derive(Clone)]
pub struct EphemeralKeyPublic (PublicKey);

impl EphemeralKeyPublic {
    pub fn generate_for_private(private: &PrivateKey) -> Self {
        EphemeralKeyPublic(
            PublicKey::from_secret_scalar(&private.0.to_nonzero_scalar())
        )
    }
}

impl Key for EphemeralKeyPublic {
    fn key(&self) -> &PublicKey {
        &self.0
    }
}

#[derive(Clone)]
pub struct SignedPreKeyPublic (PublicKey);

impl SignedPreKeyPublic {
    pub fn generate_for_private(private: &PrivateKey) -> Self {
        SignedPreKeyPublic(
            PublicKey::from_secret_scalar(&private.0.to_nonzero_scalar())
        )
    }
}

impl Key for SignedPreKeyPublic {
    fn key(&self) -> &PublicKey {
        &self.0
    }
}

#[derive(Clone)]
pub struct OneTimePreKeyPublic (PublicKey);

impl OneTimePreKeyPublic {
    pub fn generate_for_private(private: &PrivateKey) -> Self {
        OneTimePreKeyPublic(
            PublicKey::from_secret_scalar(&private.0.to_nonzero_scalar())
        )
    }
}

impl Key for OneTimePreKeyPublic {
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

pub struct X3dhSharedSecret {

}

pub struct IdentityKeyPair (IdentityKeyPublic, PrivateKey);

impl IdentityKeyPair {
    pub fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        let private = PrivateKey::generate(rng);
        IdentityKeyPair(
            IdentityKeyPublic::generate_for_private(&private),
            private,
        )
    }

    pub fn public(&self) -> IdentityKeyPublic {
        self.0.clone()
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        let sk = SigningKey::from(&self.1.0);
        Signature(sk.sign(msg))
    }

    pub fn diffie_hellman<K: Key>(&self, key: &K) -> SharedSecret {
        let sk = self.1.0.to_nonzero_scalar();
        let pk = key.key().as_affine();
        let dh = diffie_hellman(sk, pk);
        SharedSecret(dh)
    }
}

pub struct EphemeralKeyPair (EphemeralKeyPublic, PrivateKey);

impl EphemeralKeyPair {
    pub fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        let private = PrivateKey::generate(rng);
        EphemeralKeyPair(
            EphemeralKeyPublic::generate_for_private(&private),
            private,
        )
    }

    pub fn public(&self) -> EphemeralKeyPublic {
        self.0.clone()
    }

    pub fn diffie_hellman<K: Key>(self, key: &K) -> SharedSecret {
        let sk = self.1.0.to_nonzero_scalar();
        let pk = key.key().as_affine();
        let dh = diffie_hellman(sk, pk);
        SharedSecret(dh)
    }
}

pub struct SignedPreKeyPair (SignedPreKeyPublic, PrivateKey);

impl SignedPreKeyPair {
    pub fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        let private = PrivateKey::generate(rng);
        SignedPreKeyPair(
            SignedPreKeyPublic::generate_for_private(&private),
            private,
        )
    }

    pub fn public(&self) -> SignedPreKeyPublic {
        self.0.clone()
    }

    pub fn diffie_hellman<K: Key>(&self, key: &K) -> SharedSecret {
        let sk = self.1.0.to_nonzero_scalar();
        let pk = key.key().as_affine();
        let dh = diffie_hellman(sk, pk);
        SharedSecret(dh)
    }
}

pub struct OneTimeKeyPair (OneTimePreKeyPublic, PrivateKey, u16);

impl OneTimeKeyPair {
    pub fn generate<R: CryptoRng + RngCore>(rng: &mut R, index: u16) -> Self {
        let private = PrivateKey::generate(rng);
        OneTimeKeyPair(
            OneTimePreKeyPublic::generate_for_private(&private),
            private,
            index,
        )
    }

    pub fn public(&self) -> OneTimePreKeyPublic {
        self.0.clone()
    }

    pub fn diffie_hellman<K: Key>(&self, key: &K) -> SharedSecret {
        let sk = self.1.0.to_nonzero_scalar();
        let pk = key.key().as_affine();
        let dh = diffie_hellman(sk, pk);
        SharedSecret(dh)
    }
}

pub struct PeerBundle {
    pub identity: IdentityKeyPublic,
    pub signed_pre: SignedPreKeyPublic,
    pub signature:  Signature,
    pub one_time_pre: OneTimePreKeyPublic,
}

pub struct InitialMessage {
    pub identity: IdentityKeyPublic,
    pub ephemeral: EphemeralKeyPublic,
}

pub fn x3dh_sig(
    signature: &Signature,
    identity_me: &IdentityKeyPair,
    signed_pre_you: &SignedPreKeyPublic,
    ephemeral_me: &EphemeralKeyPair,
    identity_you: &IdentityKeyPublic,
    one_time_pre_you: &OneTimePreKeyPublic 
) -> X3dhSharedSecret {
    todo!();
}

pub fn x3dh(
    identity_you: &IdentityKeyPublic,
    signed_pre_me: &SignedPreKeyPair,
    ephemeral_you: &EphemeralKeyPublic,
    identity_me: &IdentityKeyPair,
    one_time_pre_me: &OneTimeKeyPair
) -> X3dhSharedSecret {
    todo!();
}
