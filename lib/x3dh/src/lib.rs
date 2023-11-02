struct Signature {

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

struct IdentityKeyPair {

}

struct EphemeralKeyPair {

}

struct SignedPreKeyPair {

}

struct OneTimeKeyPair {

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
) -> SharedSecret {
    todo!();
}

fn x3dh(
    identity_you: &IdentityKeyPublic,
    signed_pre_me: &SignedPreKeyPair,
    ephemeral_you: &EphemeralKeyPublic,
    identity_me: &IdentityKeyPair,
    one_time_pre_me: &OneTimeKeyPair
) -> SharedSecret {
    todo!();
}
