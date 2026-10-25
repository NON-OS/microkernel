//! The 48-byte opening a note seals: value, asset id and blinding, little endian.

use super::OPENING_LEN;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// What a payee needs to spend. The spend key is the payee's own, so it is not carried.
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct Opening {
    pub value: u64,
    pub asset_id: u64,
    pub blinding: [u64; 4],
}

impl Opening {
    pub(super) fn to_bytes(&self) -> [u8; OPENING_LEN] {
        let mut out = [0u8; OPENING_LEN];
        let words = [
            self.value,
            self.asset_id,
            self.blinding[0],
            self.blinding[1],
            self.blinding[2],
            self.blinding[3],
        ];
        for (slot, w) in out.chunks_exact_mut(8).zip(words.iter()) {
            slot.copy_from_slice(&w.to_le_bytes());
        }
        out
    }

    pub(super) fn from_bytes(b: &[u8; OPENING_LEN]) -> Opening {
        let mut words = [0u64; 6];
        for (w, chunk) in words.iter_mut().zip(b.chunks_exact(8)) {
            let mut le = [0u8; 8];
            le.copy_from_slice(chunk);
            *w = u64::from_le_bytes(le);
        }
        Opening {
            value: words[0],
            asset_id: words[1],
            blinding: [words[2], words[3], words[4], words[5]],
        }
    }
}
