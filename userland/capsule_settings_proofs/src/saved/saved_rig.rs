// NONOS Operating System (AGPL-3.0-or-later)

//! A key, a nonce and two saved networks, for the saved-record proofs.

use super::list::SavedList;

pub(crate) const KEY: [u8; 32] = [7; 32];
pub(crate) const NONCE: [u8; 12] = [9; 12];

pub(crate) fn two() -> SavedList {
    let mut l = SavedList::new();
    assert!(l.put(b"home", b"correct horse"));
    assert!(l.put(b"cafe", b""));
    l
}
