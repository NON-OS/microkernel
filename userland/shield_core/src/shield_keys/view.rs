//! The viewing key, which finds incoming notes and cannot spend them.
//! The agreement is crate internal, so no caller outside the core derives note keys itself.

use super::derive::byte_key;
use super::domain::VIEW_CONTEXT;
use x25519_dalek::{PublicKey, StaticSecret};
use zeroize::Zeroize;

/// The viewing key pair. The secret decrypts notes to this account and cannot spend.
pub struct ViewKey {
    secret: StaticSecret,
}

impl ViewKey {
    /// Derive the viewing secret from the seed.
    pub fn from_seed(seed: &[u8]) -> ViewKey {
        let mut bytes = byte_key(seed, VIEW_CONTEXT);
        let secret = StaticSecret::from(bytes);
        bytes.zeroize();
        ViewKey { secret }
    }

    /// The public half, which goes into the address.
    pub fn public(&self) -> [u8; 32] {
        PublicKey::from(&self.secret).to_bytes()
    }

    /// The Diffie Hellman secret with a sender's ephemeral key, used only by `notes::cipher`.
    pub(crate) fn agree(&self, ephemeral: &[u8; 32]) -> [u8; 32] {
        self.secret.diffie_hellman(&PublicKey::from(*ephemeral)).to_bytes()
    }
}

impl core::fmt::Debug for ViewKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("ViewKey(redacted)")
    }
}
