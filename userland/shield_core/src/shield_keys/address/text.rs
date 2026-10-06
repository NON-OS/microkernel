//! Writing an address.
//!
//! The payload is the spend key's four words then the viewing key, and the
//! checksum covers both under a version tag. The order is fixed here and
//! nowhere else, so changing it changes every address this wallet has issued.

use super::{checksum, Address, CHECK, PAYLOAD, PREFIX};
use crate::keys::base32::encode;

// The encoder is written for whole groups of five bytes and a three byte
// tail, which is what an address is. This is checked when the crate compiles,
// so the branch below in which the encoder has nothing to say cannot be taken.
const _: () = assert!((PAYLOAD + CHECK) % 5 == 3);

impl Address {
    pub(super) fn payload(&self) -> [u8; PAYLOAD] {
        let mut out = [0u8; PAYLOAD];
        let (keys, view) = out.split_at_mut(32);
        for (slot, word) in keys.chunks_exact_mut(8).zip(self.spend_pk.iter()) {
            slot.copy_from_slice(&word.to_le_bytes());
        }
        view.copy_from_slice(&self.view_pk);
        out
    }

    /// The address as text, prefix and all.
    pub fn to_text(&self) -> String {
        let payload = self.payload();
        let mut bytes = Vec::with_capacity(PAYLOAD + CHECK);
        bytes.extend_from_slice(&payload);
        bytes.extend_from_slice(&checksum(&payload));
        let mut text = String::from(PREFIX);
        if let Some(body) = encode(&bytes) {
            text.push_str(&body);
        }
        text
    }
}
