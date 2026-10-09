//! The note plaintext's byte form: 80 bytes, little endian, in field order.
//! It is what the AEAD seals, so any change to order or width invalidates every stored note.
//! Decoding cannot fail. The length check lives in `wire`, before the AEAD.

use super::plaintext::{NotePlaintext, PLAIN_LEN};

/// The fields in the order they are written.
const WORDS: usize = 10;

impl NotePlaintext {
    fn words(&self) -> [u64; WORDS] {
        let [b0, b1, b2, b3] = self.blinding;
        let [s0, s1, s2, s3] = self.spend_pk;
        [self.value, self.asset_id, b0, b1, b2, b3, s0, s1, s2, s3]
    }

    pub(crate) fn encode(&self) -> [u8; PLAIN_LEN] {
        let mut out = [0u8; PLAIN_LEN];
        for (slot, word) in out.chunks_exact_mut(8).zip(self.words()) {
            slot.copy_from_slice(&word.to_le_bytes());
        }
        out
    }

    pub(crate) fn decode(bytes: &[u8; PLAIN_LEN]) -> NotePlaintext {
        let mut words = [0u64; WORDS];
        for (slot, chunk) in words.iter_mut().zip(bytes.chunks_exact(8)) {
            let mut word = [0u8; 8];
            word.copy_from_slice(chunk);
            *slot = u64::from_le_bytes(word);
        }
        let [value, asset_id, b0, b1, b2, b3, s0, s1, s2, s3] = words;
        NotePlaintext { value, asset_id, blinding: [b0, b1, b2, b3], spend_pk: [s0, s1, s2, s3] }
    }
}
