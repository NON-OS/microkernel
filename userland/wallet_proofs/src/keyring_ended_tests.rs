// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! An ended owner's keys are dropped and their places reused.

use super::keyring_owner_rig::{fill_all, kept, put};
use crate::store::{Store, StoreError};

/*
 * A key answers only its owner, so once the owner ended nobody could use or
 * delete it, and its secret bytes and place were held for the boot. Eight
 * such owners filled the keyring.
 */
#[test]
fn an_ended_owner_s_keys_are_dropped_and_its_places_reused() {
    let mut s = Store::new();
    let owners = fill_all(&mut s, 100);
    assert!(owners.len() >= 2);
    assert!(s.is_full());
    let newcomer = 999;
    assert!(matches!(put(&mut s, newcomer), Err(StoreError::Full)), "the keyring is full");
    let (dead, dead_ids) = &owners[0];
    assert_eq!(s.drop_ended(|pid| pid != *dead), dead_ids.len());
    assert!(!s.is_full());
    for &id in dead_ids {
        assert!(matches!(s.retrieve(id, *dead), Err(StoreError::NotFound)), "the key is gone");
    }
    for (owner, ids) in &owners[1..] {
        assert!(ids.iter().all(|&id| kept(&mut s, *owner, id)), "a living owner's keys stay");
    }
    assert!(put(&mut s, newcomer).is_ok(), "a new owner is served");
}
