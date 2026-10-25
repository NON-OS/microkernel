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

//! The store-to-wallet mailbox (op 19, plan/27): names only, the store's
//! alone to post, one held for ten minutes, taken once by the wallet.

use nonos_nox_meter::caller::Role;
use nonos_nox_meter::mailbox::{Mailbox, HOLD_MS};
use nonos_payment_proto::{Names, EBUSY};

fn names(listing: &[u8]) -> Names {
    Names { chain_id: 11_155_111, listing: listing.to_vec(), release: Vec::new() }
}

fn registry(name: &[u8]) -> Option<u32> {
    match name {
        b"app.store" => Some(31),
        b"app.nonos_wallet.1" => Some(10),
        b"installer" => Some(30),
        _ => None,
    }
}

#[test]
fn only_the_store_posts() {
    assert!(Role::Store.admits(31, registry));
    assert!(!Role::Store.admits(10, registry), "not the wallet");
    assert!(!Role::Store.admits(30, registry), "not the installer");
}

#[test]
fn one_waits_at_a_time_and_is_taken_once() {
    let mut m = Mailbox::default();
    assert_eq!(m.names.take(0), None);
    m.names.post(names(b"linux.a"), 0).unwrap();
    assert_eq!(m.names.post(names(b"linux.b"), 1), Err(EBUSY), "the first still waits");
    assert_eq!(m.names.take(2), Some(names(b"linux.a")));
    assert_eq!(m.names.take(3), None, "taken once, then gone");
    m.names.post(names(b"linux.b"), 4).unwrap();
}

#[test]
fn names_left_ten_minutes_are_dropped_and_make_room() {
    let mut m = Mailbox::default();
    m.names.post(names(b"linux.a"), 0).unwrap();
    m.names.post(names(b"linux.b"), HOLD_MS).unwrap();
    assert_eq!(m.names.take(HOLD_MS * 2), None, "too old to take");
    assert_eq!(m.names.take(HOLD_MS * 2), None);
}
