// NONOS Operating System (AGPL-3.0-or-later)
//! A default store is the keyring's fresh one: empty, numbering from 1.

use super::keyring_owner_rig::{fill_all, put, A};
use crate::store::Store;

#[test]
fn a_default_store_is_empty_and_numbers_its_keys_from_one() {
    let mut s = Store::default();
    assert_eq!(s.count_owned_by(A), 0);
    assert!(!s.is_full());
    assert_eq!(put(&mut s, A).ok(), Some(1));
    assert_eq!(put(&mut s, A).ok(), Some(2));
    assert_eq!(s.count_owned_by(A), 2);
}

#[test]
fn a_default_store_holds_what_a_new_one_holds() {
    let made = fill_all(&mut Store::default(), 100);
    assert_eq!(made, fill_all(&mut Store::new(), 100));
}
