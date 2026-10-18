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

//! Each slot of the op 19 mailbox holds a posting for exactly HOLD_MS of
//! the clock the capsule passes (monotonic, mk_uptime_ms): live at
//! HOLD_MS - 1 after the POST, gone at HOLD_MS.

use core::fmt::Debug;

use nonos_nox_meter::mailbox::{Mailbox, Slot, HOLD_MS};
use nonos_payment_proto::{Names, Ready, EBUSY};

const T: u64 = 5_000;

fn names(listing: &[u8]) -> Names {
    Names { chain_id: 11_155_111, listing: listing.to_vec(), release: Vec::new() }
}

fn holds_for_hold_ms<T2: Clone + PartialEq + Debug>(
    slot: fn(&mut Mailbox) -> &mut Slot<T2>,
    a: T2,
    b: T2,
) {
    let mut m = Mailbox::default();
    slot(&mut m).post(a.clone(), T).unwrap();
    assert_eq!(slot(&mut m).take(T + HOLD_MS - 1), Some(a.clone()), "live 1 ms before");
    let mut m = Mailbox::default();
    slot(&mut m).post(a.clone(), T).unwrap();
    assert_eq!(slot(&mut m).take(T + HOLD_MS), None, "gone at HOLD_MS");
    let mut m = Mailbox::default();
    slot(&mut m).post(a, T).unwrap();
    assert_eq!(slot(&mut m).post(b.clone(), T + HOLD_MS - 1), Err(EBUSY));
    assert_eq!(slot(&mut m).post(b.clone(), T + HOLD_MS), Ok(()), "room at HOLD_MS");
    assert_eq!(slot(&mut m).take(T + HOLD_MS), Some(b), "the new posting");
}

#[test]
fn names_wait_until_one_ms_before_hold_ms_and_no_longer() {
    holds_for_hold_ms(|m| &mut m.names, names(b"linux.a"), names(b"linux.b"));
}

#[test]
fn the_order_id_waits_until_one_ms_before_hold_ms_and_no_longer() {
    let r = |l: &[u8], id| Ready { names: names(l), order_id: [id; 32] };
    holds_for_hold_ms(|m| &mut m.ready, r(b"linux.a", 1), r(b"linux.b", 2));
}

/// Why the clock must be monotonic: a `now` behind the posting counts as
/// no time at all, so a wall clock stepped back would hold it (and EBUSY)
/// past HOLD_MS.
#[test]
fn a_now_behind_the_posting_counts_as_no_time() {
    let mut m = Mailbox::default();
    m.names.post(names(b"linux.a"), HOLD_MS * 3).unwrap();
    assert_eq!(m.names.post(names(b"linux.b"), 0), Err(EBUSY));
    assert_eq!(m.names.take(HOLD_MS), Some(names(b"linux.a")));
}
