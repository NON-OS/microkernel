//! What the keys of account i are derived from. Account 0 takes the seed alone, so every wallet
//! keeps the keys and notes it has today. Account i after it takes the seed, `account` and i as
//! four little endian bytes, so no key of one account is computable from another.

use crate::custody::Seed;
use zeroize::Zeroizing;

/// The bytes every key of one account is derived from, wiped on drop.
pub(crate) struct Material(Zeroizing<Vec<u8>>);

impl Material {
    pub(crate) fn of(seed: &Seed, index: u32) -> Material {
        let mut bytes = Zeroizing::new(Vec::with_capacity(64 + 7 + 4));
        bytes.extend_from_slice(seed.bytes());
        if index > 0 {
            bytes.extend_from_slice(b"account");
            bytes.extend_from_slice(&index.to_le_bytes());
        }
        Material(bytes)
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        &self.0
    }
}
