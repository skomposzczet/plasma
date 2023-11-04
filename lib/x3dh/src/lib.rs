pub mod keys;
pub mod handshake;
pub mod error;

use keys::{Signature, IdentityKeyPair, SignedPreKeyPublic, EphemeralKeyPair, IdentityKeyPublic, OneTimePreKeyPublic, X3dhSharedSecret, SignedPreKeyPair, EphemeralKeyPublic, OneTimeKeyPair};

pub fn x3dh_sig(
    signature: &Signature,
    identity_me: &IdentityKeyPair,
    signed_pre_you: &SignedPreKeyPublic,
    ephemeral_me: &EphemeralKeyPair,
    identity_you: &IdentityKeyPublic,
    one_time_pre_you: &OneTimePreKeyPublic
) -> X3dhSharedSecret {
    todo!()
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
