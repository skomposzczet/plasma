use crate::keys::{Signature, SignedPreKeyPublic, IdentityKeyPublic, OneTimePreKeyPublic, EphemeralKeyPublic};

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

