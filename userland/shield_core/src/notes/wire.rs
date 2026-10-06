//! A note ciphertext as it travels: ephemeral key, view tag, sealed plaintext, fixed length.
//! Any other length is refused before the AEAD, so a malformed record is a parse failure.

use super::plaintext::PLAIN_LEN;
use nonos_seal::TAG_LEN;

/// A note ciphertext on the wire.
pub const CIPHER_LEN: usize = 32 + 1 + PLAIN_LEN + TAG_LEN;

/// The sealed note beside a commitment. It names the recipient only to the shared secret holder.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NoteCipher {
    pub eph_pk: [u8; 32],
    pub view_tag: u8,
    pub sealed: [u8; PLAIN_LEN + TAG_LEN],
}

impl NoteCipher {
    /// The ciphertext as fixed length bytes.
    pub fn encode(&self) -> [u8; CIPHER_LEN] {
        let mut out = [0u8; CIPHER_LEN];
        let (key, rest) = out.split_at_mut(32);
        key.copy_from_slice(&self.eph_pk);
        let (tag, body) = rest.split_at_mut(1);
        tag.copy_from_slice(&[self.view_tag]);
        body.copy_from_slice(&self.sealed);
        out
    }

    /// Read a ciphertext, refusing any length but the fixed one.
    pub fn decode(bytes: &[u8]) -> Option<NoteCipher> {
        if bytes.len() != CIPHER_LEN {
            return None;
        }
        let (key, rest) = bytes.split_at_checked(32)?;
        let (tag, body) = rest.split_first()?;
        Some(NoteCipher {
            eph_pk: key.try_into().ok()?,
            view_tag: *tag,
            sealed: body.try_into().ok()?,
        })
    }
}
