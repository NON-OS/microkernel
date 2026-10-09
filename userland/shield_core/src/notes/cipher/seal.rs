//! Sealing a note to a recipient. The ephemeral secret is dropped here, so not even the sender
//! can reopen it, and a seized phone does not reveal the recipient's balance.
//! The nonce is fixed because the key is used once. It stays safe only while the KDF binds the
//! ephemeral public key.

use super::super::kdf::{note_key, view_tag};
use super::super::plaintext::{NotePlaintext, PLAIN_LEN};
use super::super::wire::NoteCipher;
use super::NONCE;
use crate::entropy::fill;
use crate::error::CustodyError;
use crate::keys::Address;
use nonos_seal::{seal, TAG_LEN};
use x25519_dalek::{PublicKey, StaticSecret};
use zeroize::Zeroize;

/// Seal a note to an address with a fresh ephemeral secret, dropped on return.
pub fn seal_note(
    plain: &NotePlaintext,
    to: &Address,
    cm_bytes: &[u8],
) -> Result<NoteCipher, CustodyError> {
    let mut eph_bytes = [0u8; 32];
    fill(&mut eph_bytes)?;
    let eph = StaticSecret::from(eph_bytes);
    eph_bytes.zeroize();
    let eph_pk = PublicKey::from(&eph).to_bytes();
    let mut shared = eph.diffie_hellman(&PublicKey::from(to.view_pk)).to_bytes();
    let mut key = note_key(&shared, &eph_pk, &to.view_pk);
    let tag = view_tag(&shared, &eph_pk);
    shared.zeroize();

    let mut body = plain.encode();
    let mut sealed = [0u8; PLAIN_LEN + TAG_LEN];
    let wrote = seal(&key, &NONCE, cm_bytes, &body, &mut sealed);
    key.zeroize();
    body.zeroize();
    if wrote != Ok(PLAIN_LEN + TAG_LEN) {
        return Err(CustodyError::SealAuth);
    }
    Ok(NoteCipher { eph_pk, view_tag: tag, sealed })
}
