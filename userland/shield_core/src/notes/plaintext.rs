//! What a note says once its ciphertext opens, in a fixed 80 byte form so sizes reveal nothing.
//! It zeroes on drop, is not Copy, and its Debug hides the amount and blinding.

use stark_proofs::shield::note::Note;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Plaintext bytes on the wire: value, asset, blinding, spend key, each little endian.
pub const PLAIN_LEN: usize = 8 + 8 + 32 + 32;

/// What a recipient learns when a note opens: enough to rebuild the commitment and spend it.
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct NotePlaintext {
    pub value: u64,
    pub asset_id: u64,
    pub blinding: [u64; 4],
    pub spend_pk: [u64; 4],
}

impl NotePlaintext {
    /// The note as the shield stack's own type, for commitments and witnesses.
    pub fn note(&self) -> Note {
        Note {
            value: self.value,
            asset_id: self.asset_id,
            spend_pk: self.spend_pk,
            blinding: self.blinding,
        }
    }
}

impl core::fmt::Debug for NotePlaintext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("NotePlaintext(redacted)")
    }
}
