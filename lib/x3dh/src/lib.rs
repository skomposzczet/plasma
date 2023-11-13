pub mod keys;
pub mod handshake;
pub mod error;

use hkdf::Hkdf;
use error::X3dhError;
use keys::{Signature, IdentityKeyPair, SignedPreKeyPublic, EphemeralKeyPair, IdentityKeyPublic, OneTimePreKeyPublic, X3dhSharedSecret, SignedPreKeyPair, EphemeralKeyPublic, OneTimeKeyPair, Key, SharedSecret, KeyPair};

fn dh_to_shared(
    dh1: &SharedSecret,
    dh2: &SharedSecret,
    dh3: &SharedSecret,
    dh4: &SharedSecret,
) -> X3dhSharedSecret {
    let mut data = Vec::new();
    let mut null = [0u8; 32].to_vec();
    data.append(&mut null);
    data.append(&mut dh1.to_bytes());
    data.append(&mut dh2.to_bytes());
    data.append(&mut dh3.to_bytes());
    data.append(&mut dh4.to_bytes());

    let h = Hkdf::<sha2::Sha512>::new(Some(&[0u8; 32]), &data);
    let mut okm = [0u8; 32];
    h.expand(b"x3dh", &mut okm).unwrap();
    X3dhSharedSecret::from_bytes(&okm)
}

pub fn x3dh_sig(
    signature: &Signature,
    identity_me: &IdentityKeyPair,
    signed_pre_you: &SignedPreKeyPublic,
    ephemeral_me: &EphemeralKeyPair,
    identity_you: &IdentityKeyPublic,
    one_time_pre_you: &OneTimePreKeyPublic
) -> Result<X3dhSharedSecret, X3dhError> {
    identity_you.verify(&signed_pre_you.key().to_sec1_bytes(), signature)?;

    let dh1 = identity_me.diffie_hellman(signed_pre_you);
    let dh2 = ephemeral_me.diffie_hellman(identity_you);
    let dh3 = ephemeral_me.diffie_hellman(signed_pre_you);
    let dh4 = ephemeral_me.diffie_hellman(one_time_pre_you);

    Ok(dh_to_shared(&dh1, &dh2, &dh3, &dh4))
}

pub fn x3dh(
    identity_you: &IdentityKeyPublic,
    signed_pre_me: &SignedPreKeyPair,
    ephemeral_you: &EphemeralKeyPublic,
    identity_me: &IdentityKeyPair,
    one_time_pre_me: &OneTimeKeyPair
) -> X3dhSharedSecret {
    let dh1 = signed_pre_me.diffie_hellman(identity_you);
    let dh2 = identity_me.diffie_hellman(ephemeral_you);
    let dh3 = signed_pre_me.diffie_hellman(ephemeral_you);
    let dh4 = one_time_pre_me.diffie_hellman(ephemeral_you);

    dh_to_shared(&dh1, &dh2, &dh3, &dh4)
}

#[cfg(test)]
mod x3dh_test {
    use crate::{keys::{IdentityKeyPair, EphemeralKeyPair, SignedPreKeyPair, OneTimeKeyPair, KeyPair, Key}, x3dh_sig, x3dh};

    #[test]
    fn x3dh_protocol_test() {
        let mut rng = rand::rngs::OsRng::default();

        let ika = IdentityKeyPair::generate(&mut rng);
        let eka = EphemeralKeyPair::generate(&mut rng);

        let ikb = IdentityKeyPair::generate(&mut rng);
        let spb = SignedPreKeyPair::generate(&mut rng);
        let opb = OneTimeKeyPair::generate(&mut rng).with_index(0);
        let sig = ikb.sign(&spb.public().key().to_sec1_bytes());

        let ssa = x3dh_sig(
            &sig,
            &ika,
            &spb.public(),
            &eka,
            &ikb.public(),
            &opb.public(),
        ).unwrap();
        let ssb = x3dh(
            &ika.public(),
            &spb,
            &eka.public(),
            &ikb,
            &opb,
        );

        assert_eq!(ssa.to_bytes(), ssb.to_bytes());
    }
}
