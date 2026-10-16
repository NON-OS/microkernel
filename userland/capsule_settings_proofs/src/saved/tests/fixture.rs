// NONOS Operating System (AGPL-3.0-or-later)

//! The key, nonce and two-network list the record proofs share.

use super::super::list::SavedList;

pub(super) const KEY: [u8; 32] = [7; 32];

pub(super) const NONCE: [u8; 12] = [9; 12];

pub(super) fn two() -> SavedList {
    let mut l = SavedList::new();
    assert!(l.put(b"home", b"correct horse"));
    assert!(l.put(b"cafe", b""));
    l
}
