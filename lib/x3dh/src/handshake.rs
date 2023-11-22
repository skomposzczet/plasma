use serde::{Serialize, Deserialize};
use crate::keys::{Signature, SignedPreKeyPublic, IdentityKeyPublic, OneTimePreKeyPublic, OneTimeKeyPair, EphemeralKeyPublic, KeyPair, Key};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneTimePreKeyPublicBundle (OneTimePreKeyPublic, u16);

impl OneTimePreKeyPublicBundle {
    pub fn from_pair(key: &OneTimeKeyPair) -> Self {
        OneTimePreKeyPublicBundle(key.public().clone(), key.index())
    }

    pub fn key(&self) -> &OneTimePreKeyPublic {
        &self.0
    }

    pub fn index(&self) -> u16 {
        self.1
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&(self.0.to_bytes(), self.1)).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let (key, idx): (Vec<u8>, u16) = bincode::deserialize(bytes).unwrap();
        OneTimePreKeyPublicBundle(OneTimePreKeyPublic::from_bytes(&key), idx)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterBundle {
    pub identity: IdentityKeyPublic,
    pub signed_pre: SignedPreKeyPublic,
    pub signature: Signature,
    pub one_time_pres: Vec<OneTimePreKeyPublicBundle>,
}

impl RegisterBundle {
    pub fn serialize(&self) -> RegisterBundleBinary {
        RegisterBundleBinary {
            identity: self.identity.to_bytes(),
            signed_pre: self.signed_pre.to_bytes(),
            signature: self.signature.to_bytes(),
            one_time_pres: self.one_time_pres.iter()
                .map(|key| key.to_bytes())
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RegisterBundleBinary {
    identity: Vec<u8>,
    signed_pre: Vec<u8>,
    signature: Vec<u8>,
    one_time_pres: Vec<Vec<u8>>,
}

impl RegisterBundleBinary {
    pub fn deserialize(self) -> RegisterBundle {
        RegisterBundle {
            identity: IdentityKeyPublic::from_bytes(&self.identity),
            signed_pre: SignedPreKeyPublic::from_bytes(&self.signed_pre),
            signature: Signature::from_bytes(&self.signature),
            one_time_pres: self.one_time_pres.iter()
                .map(|bytes| OneTimePreKeyPublicBundle::from_bytes(bytes))
                .collect()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerBundle {
    pub identity: IdentityKeyPublic,
    pub signed_pre: SignedPreKeyPublic,
    pub signature:  Signature,
    pub one_time_pre: OneTimePreKeyPublicBundle,
}

impl PeerBundle {
    pub fn serialize(&self) -> PeerBundleBinary {
        PeerBundleBinary {
            identity: self.identity.to_bytes(),
            signed_pre: self.signed_pre.to_bytes(),
            signature: self.signature.to_bytes(),
            one_time_pre: self.one_time_pre.to_bytes(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PeerBundleBinary {
    identity: Vec<u8>,
    signed_pre: Vec<u8>,
    signature: Vec<u8>,
    one_time_pre: Vec<u8>,
}

impl PeerBundleBinary {
    pub fn deserialize(self) -> PeerBundle {
        PeerBundle {
            identity: IdentityKeyPublic::from_bytes(&self.identity),
            signed_pre: SignedPreKeyPublic::from_bytes(&self.signed_pre),
            signature: Signature::from_bytes(&self.signature),
            one_time_pre: OneTimePreKeyPublicBundle::from_bytes(&self.one_time_pre)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitialMessage {
    pub identity: IdentityKeyPublic,
    pub ephemeral: EphemeralKeyPublic,
    pub one_time_idx: u16,
}

impl InitialMessage {
    pub fn serialize(&self) -> InitialMessageBinary {
        InitialMessageBinary {
            identity: self.identity.to_bytes(),
            ephemeral: self.ephemeral.to_bytes(),
            one_time_idx: self.one_time_idx,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct InitialMessageBinary {
    identity: Vec<u8>,
    ephemeral: Vec<u8>,
    one_time_idx: u16,
}

impl InitialMessageBinary {
    pub fn deserialize(self) -> InitialMessage {
        InitialMessage {
            identity: IdentityKeyPublic::from_bytes(&self.identity),
            ephemeral: EphemeralKeyPublic::from_bytes(&self.ephemeral),
            one_time_idx: self.one_time_idx,
        }
    }
}

#[cfg(test)]
mod handshake_test {
    use rand::Rng;

    use crate::keys::{IdentityKeyPair, KeyPair, SignedPreKeyPair, Key, OneTimeKeyPair, EphemeralKeyPair};

    use super::{RegisterBundle, OneTimePreKeyPublicBundle, PeerBundle, InitialMessage};

    fn random_register_bundle() -> RegisterBundle {
        let mut rng = rand::rngs::OsRng::default();
        let identity = IdentityKeyPair::generate(&mut rng);
        let signed_pre = SignedPreKeyPair::generate(&mut rng);
        let sig = identity.sign(&signed_pre.public().to_bytes());
        RegisterBundle {
            identity: identity.public().clone(),
            signed_pre: signed_pre.public().clone(),
            signature: sig,
            one_time_pres: vec![OneTimePreKeyPublicBundle::from_pair(&OneTimeKeyPair::generate(&mut rng))],
        }
    }

    fn random_peer_bundle() -> PeerBundle {
        let b = random_register_bundle();
        PeerBundle {
            identity: b.identity,
            signed_pre: b.signed_pre,
            signature: b.signature,
            one_time_pre: b.one_time_pres[0].clone(),
        }
    }

    fn random_initial_message() -> InitialMessage {
        let mut rng = rand::rngs::OsRng::default();
        InitialMessage {
            identity: IdentityKeyPair::generate(&mut rng).public().clone(),
            ephemeral: EphemeralKeyPair::generate(&mut rng).public().clone(),
            one_time_idx: rand::thread_rng().gen(),
        }
    }

    #[test]
    fn register_bundle_deserialize_serialize() {
        let rb = random_register_bundle();
        let rb_clone = rb.serialize().deserialize();

        assert_eq!(rb, rb_clone);
    }

    #[test]
    fn peer_bundl_deserialize_serialize () {
        let pb = random_peer_bundle();
        let pb_clone = pb.serialize().deserialize();

        assert_eq!(pb, pb_clone);
    }

    #[test]
    fn initial_message_deserialize_serialize() {
        let im = random_initial_message();
        let im_clone = im.serialize().deserialize();

        assert_eq!(im, im_clone);
    }
}
