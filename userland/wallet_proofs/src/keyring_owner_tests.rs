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

//! The keyring's key store: one owner holds only its share, and the keys of
//! an owner that ended are dropped while a living owner's stay.

use crate::store::{KeyType, Store, StoreError, MAX_KEYS, MAX_KEYS_PER_OWNER};

const A: u32 = 70;
const B: u32 = 71;
const SECRET: [u8; 32] = [0x11; 32];

fn put(s: &mut Store, owner: u32) -> Result<u32, StoreError> {
    s.store(KeyType::Symmetric, &SECRET, owner, 0, 0)
}

/// Store keys for `owner` until the store refuses it.
fn fill(s: &mut Store, owner: u32) -> Vec<u32> {
    let mut ids = Vec::new();
    while let Ok(id) = put(s, owner) {
        ids.push(id);
    }
    ids
}

/// Fill the whole store, one owner after another from `first`.
fn fill_all(s: &mut Store, first: u32) -> Vec<(u32, Vec<u32>)> {
    let mut owners = Vec::new();
    let mut owner = first;
    loop {
        let ids = fill(s, owner);
        if ids.is_empty() {
            return owners;
        }
        owners.push((owner, ids));
        owner += 1;
    }
}

fn kept(s: &mut Store, owner: u32, id: u32) -> bool {
    matches!(s.retrieve(id, owner), Ok(bytes) if bytes == SECRET)
}

#[test]
fn an_owner_stops_at_its_share_and_another_still_stores() {
    let mut s = Store::new();
    assert_eq!(fill(&mut s, A).len(), MAX_KEYS_PER_OWNER);
    assert!(matches!(put(&mut s, A), Err(StoreError::Full)), "past its share");
    assert!(put(&mut s, B).is_ok(), "another owner still stores");
    let total: usize = fill_all(&mut Store::new(), 100).iter().map(|(_, ids)| ids.len()).sum();
    assert_eq!(total, MAX_KEYS);
}

#[test]
#[allow(clippy::assertions_on_constants)] // the relation is the point
fn one_owner_never_holds_every_key() {
    assert!(MAX_KEYS_PER_OWNER < MAX_KEYS);
}

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

#[test]
fn nobody_ended_drops_nothing() {
    let mut s = Store::new();
    let id = put(&mut s, A).ok().unwrap_or(0);
    assert_eq!(s.drop_ended(|_| true), 0);
    assert!(kept(&mut s, A, id));
}
