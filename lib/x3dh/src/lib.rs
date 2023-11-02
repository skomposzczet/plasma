use rand::{CryptoRng, RngCore};

struct Signature {

}

struct PublicKey {

}

trait Key {
    fn key(&self) -> PublicKey;
}

struct IdentityKeyPublic {

}

struct EphemeralKeyPublic {

}

struct SignedPreKeyPublic {

}

struct OneTimePreKeyPublic {

}

struct PrivateKey {

}

struct SharedSecret {

}

struct X3dhSharedSecret {

}

struct IdentityKeyPair {

}

impl IdentityKeyPair {
    fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        todo!();
    }

    fn public(&self) -> IdentityKeyPublic {
        todo!();
    }

    fn sign(&self, msg: &[u8]) -> Signature {
        todo!();
    }

    fn diffie_hellman<K: Key>(&self, key: &K) -> SharedSecret {
        todo!();
    }
}

struct EphemeralKeyPair {

}

impl EphemeralKeyPair {
    fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        todo!();
    }

    fn public(&self) -> IdentityKeyPublic {
        todo!();
    }

    fn diffie_hellman<K: Key>(self, key: &K) -> SharedSecret {
        todo!();
    }
}

struct SignedPreKeyPair {

}

impl SignedPreKeyPair {
    fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        todo!();
    }

    fn public(&self) -> IdentityKeyPublic {
        todo!();
    }

    fn diffie_hellman<K: Key>(&self, key: &K) -> SharedSecret {
        todo!();
    }
}

struct OneTimeKeyPair {

}

impl OneTimeKeyPair {
    fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        todo!();
    }

    fn public(&self) -> IdentityKeyPublic {
        todo!();
    }

    fn diffie_hellman<K: Key>(&self, key: &K) -> SharedSecret {
        todo!();
    }
}

struct PeerBundle {
    identity: IdentityKeyPublic,
    signed_pre: SignedPreKeyPublic,
    signature:  Signature,
    one_time_pre: OneTimePreKeyPublic,
}

struct InitialMessage {
    identity: IdentityKeyPublic,
    ephemeral: EphemeralKeyPublic,
}

fn x3dh_sig(
    signature: &Signature,
    identity_me: &IdentityKeyPair,
    signed_pre_you: &SignedPreKeyPublic,
    ephemeral_me: &EphemeralKeyPair,
    identity_you: &IdentityKeyPublic,
    one_time_pre_you: &OneTimePreKeyPublic 
) -> X3dhSharedSecret {
    todo!();
}

fn x3dh(
    identity_you: &IdentityKeyPublic,
    signed_pre_me: &SignedPreKeyPair,
    ephemeral_you: &EphemeralKeyPublic,
    identity_me: &IdentityKeyPair,
    one_time_pre_me: &OneTimeKeyPair
) -> X3dhSharedSecret {
    todo!();
}
