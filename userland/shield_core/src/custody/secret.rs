//! The seed, the root of every key the wallet holds.
//! It zeroes on drop, its Debug prints nothing, and its bytes never leave this crate.

use zeroize::{Zeroize, ZeroizeOnDrop};

/// The BIP39 seed, 64 bytes, zeroed on drop. Nothing derived from it is cached.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Seed {
    bytes: [u8; 64],
}

impl Seed {
    /// Wrap raw seed bytes. The caller's copy should be zeroed after this.
    pub fn new(bytes: [u8; 64]) -> Seed {
        Seed { bytes }
    }

    /// The seed bytes, crate internal so no caller outside the core can read them.
    pub(crate) fn bytes(&self) -> &[u8; 64] {
        &self.bytes
    }
}

impl core::fmt::Debug for Seed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Seed(redacted)")
    }
}
