//! Trying a note ciphertext as ours. The view tag is checked before the AEAD, so an output
//! that is not ours costs a key agreement and a byte compare, not a full decryption.

use super::super::kdf::{note_key, view_tag};
use super::super::plaintext::{NotePlaintext, PLAIN_LEN};
use super::super::wire::NoteCipher;
use super::NONCE;
use crate::keys::ViewKey;
use nonos_seal::open as aead_open;
use zeroize::Zeroize;

/// How far an output got before it was ruled out. A scan reports each, to measure the tag filter.
pub enum Opened {
    /// The view tag did not match. No decryption was attempted.
    TagMiss,
    /// The tag matched and the ciphertext did not authenticate: a collision or a misplaced record.
    AuthFail,
    /// The note, in the clear, for this wallet.
    Note(NotePlaintext),
}

/// Try to open a note ciphertext as this account's, view tag first.
pub fn open_note(cipher: &NoteCipher, view: &ViewKey, cm_bytes: &[u8]) -> Opened {
    let mut shared = view.agree(&cipher.eph_pk);
    if view_tag(&shared, &cipher.eph_pk) != cipher.view_tag {
        shared.zeroize();
        return Opened::TagMiss;
    }
    let mut key = note_key(&shared, &cipher.eph_pk, &view.public());
    shared.zeroize();
    let mut body = [0u8; PLAIN_LEN];
    let opened = aead_open(&key, &NONCE, cm_bytes, &cipher.sealed, &mut body);
    key.zeroize();
    let plain = match opened {
        Ok(PLAIN_LEN) => NotePlaintext::decode(&body),
        _ => return Opened::AuthFail,
    };
    body.zeroize();
    Opened::Note(plain)
}
