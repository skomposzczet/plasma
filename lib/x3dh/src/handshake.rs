use serde::{Serialize, Deserialize};
use crate::keys::{Signature, SignedPreKeyPublic, IdentityKeyPublic, OneTimePreKeyPublic, OneTimeKeyPair, EphemeralKeyPublic, KeyPair, Key};

pub struct OneTimePreKeyPublicBundle (OneTimePreKeyPublic, u16);

impl OneTimePreKeyPublicBundle {
    pub fn from_pair(key: &OneTimeKeyPair) -> Self {
        OneTimePreKeyPublicBundle(key.public().clone(), key.index())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&(self.0.to_bytes(), self.1)).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let (key, idx): (Vec<u8>, u16) = bincode::deserialize(bytes).unwrap();
        OneTimePreKeyPublicBundle(OneTimePreKeyPublic::from_bytes(&key), idx)
    }
}

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

pub struct InitialMessage {
    pub identity: IdentityKeyPublic,
    pub ephemeral: EphemeralKeyPublic,
}

impl InitialMessage {
    pub fn serialize(&self) -> InitialMessageBinary {
        InitialMessageBinary {
            identity: self.identity.to_bytes(),
            ephemeral: self.ephemeral.to_bytes(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct InitialMessageBinary {
    identity: Vec<u8>,
    ephemeral: Vec<u8>,
}

impl InitialMessageBinary {
    pub fn deserialize(self) -> InitialMessage {
        InitialMessage {
            identity: IdentityKeyPublic::from_bytes(&self.identity),
            ephemeral: EphemeralKeyPublic::from_bytes(&self.ephemeral),
        }
    }
}

