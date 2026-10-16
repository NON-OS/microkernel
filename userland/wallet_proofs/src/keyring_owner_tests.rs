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

use super::keyring_owner_rig::{fill, fill_all, kept, put, A, B};
use crate::store::{Store, StoreError, MAX_KEYS, MAX_KEYS_PER_OWNER};

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

#[test]
fn nobody_ended_drops_nothing() {
    let mut s = Store::new();
    let id = put(&mut s, A).ok().unwrap_or(0);
    assert_eq!(s.drop_ended(|_| true), 0);
    assert!(kept(&mut s, A, id));
}
