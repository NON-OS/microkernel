//! The address a sender needs: the spend key a note commits to, and the
//! viewing key its ciphertext is sealed to. Both halves are public.

mod parse;
mod text;

use super::domain::ADDRESS_TAG;

/// The human readable prefix, so an address pasted anywhere is recognisable and
/// one from another chain cannot be mistaken for one of these.
pub(super) const PREFIX: &str = "nox1";

/// The payload: the spend key as four field words, then the viewing key.
pub(super) const PAYLOAD: usize = 32 + 32;
pub(super) const CHECK: usize = 4;

/// A recipient: what a sender needs to build a note only that account can spend
/// and only that account can find.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Address {
    pub spend_pk: [u64; 4],
    pub view_pk: [u8; 32],
}

pub(super) fn checksum(payload: &[u8]) -> [u8; CHECK] {
    let mut h = blake3::Hasher::new();
    h.update(ADDRESS_TAG);
    h.update(payload);
    let mut out = [0u8; CHECK];
    out.copy_from_slice(&h.finalize().as_bytes()[..CHECK]);
    out
}
