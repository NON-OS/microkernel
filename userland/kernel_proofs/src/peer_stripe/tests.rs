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

use super::stripe::{hold, stripe_of, STRIPES};
use std::cell::Cell;

#[test]
fn every_asid_has_a_lock_and_neighbours_do_not_share_one() {
    assert!((0..100_000u32).all(|asid| stripe_of(asid) < STRIPES));
    assert!((0..STRIPES as u32).all(|a| stripe_of(a) != stripe_of(a + 1)));
    assert_eq!(stripe_of(7), stripe_of(7 + STRIPES as u32), "the set is fixed");
}

#[test]
fn a_guest_found_and_still_there_is_held() {
    let taken = Cell::new(None);
    let held = hold(|| Some(9), |stripe| taken.set(Some(stripe)));
    assert_eq!(held.map(|h| h.0), Some(9));
    assert_eq!(taken.get(), Some(stripe_of(9)));
}

#[test]
fn a_guest_torn_down_during_the_wait_is_not_used() {
    // The teardown held the lock first and stopped naming the address space
    // before it freed it: the call must not go on with the freed one.
    let named = Cell::new(Some(9));
    let held = hold(|| named.get(), |_| named.set(None));
    assert!(held.is_none());
}

#[test]
fn a_guest_moved_during_the_wait_is_held_under_its_new_lock() {
    let named = Cell::new(Some(9));
    let takes = Cell::new(Vec::new());
    let take = |stripe| {
        let mut seen = takes.take();
        seen.push(stripe);
        takes.set(seen);
        if named.get() == Some(9) {
            named.set(Some(10));
        }
    };
    assert_eq!(hold(|| named.get(), take).map(|h| h.0), Some(10));
    assert_eq!(takes.take(), vec![stripe_of(9), stripe_of(10)]);
}

#[test]
fn no_guest_takes_no_lock() {
    let taken = Cell::new(false);
    assert!(hold(|| None, |_| taken.set(true)).is_none());
    assert!(!taken.get());
}
