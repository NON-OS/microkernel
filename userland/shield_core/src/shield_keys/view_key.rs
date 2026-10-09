//! The view keys. `noxivk1` holds the X-Wing seed and `spend_pk` and finds every note paid in.
//! `noxfvk1` adds `nk` and so finds spent notes too. Neither holds `sk`, which spending needs.
//! A view key cannot be revoked. The only way out is to move the value to a new account.

use super::domain::VIEW_KEY_TAG;

pub(super) const VERSION: u8 = 0x01;
pub(super) const CHECK: usize = 4;
pub(super) const INCOMING: &str = "noxivk1";
pub(super) const FULL: &str = "noxfvk1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewKind {
    Incoming,
    /// Every note received and which are spent, so the balance and its history.
    Full,
}

pub(super) fn prefix(kind: ViewKind) -> &'static str {
    match kind {
        ViewKind::Incoming => INCOMING,
        ViewKind::Full => FULL,
    }
}

pub(super) fn checksum(kind: ViewKind, bytes: &[u8]) -> [u8; CHECK] {
    let mut h = blake3::Hasher::new();
    h.update(VIEW_KEY_TAG);
    h.update(prefix(kind).as_bytes());
    h.update(bytes);
    let mut out = [0u8; CHECK];
    out.copy_from_slice(h.finalize().as_bytes().get(..CHECK).unwrap_or(&[0; CHECK]));
    out
}

/// Bytes before the checksum: version, X-Wing seed, `spend_pk`, and `nk` for a full key.
pub(super) fn body(kind: ViewKind) -> usize {
    match kind {
        ViewKind::Incoming => 1 + 32 + 32,
        ViewKind::Full => 1 + 32 + 32 + 32,
    }
}

/// Zero padded to whole five-byte groups, which the address alphabet encodes.
pub(super) fn padded(kind: ViewKind) -> usize {
    body(kind).saturating_add(CHECK).div_ceil(5).saturating_mul(5)
}

pub(super) fn push_words(out: &mut Vec<u8>, words: &[nonos_stark::field::Fp; 4]) {
    for w in words {
        out.extend_from_slice(&w.value().to_le_bytes());
    }
}
