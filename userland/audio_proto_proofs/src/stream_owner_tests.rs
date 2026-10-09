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

//! A stream answers only the client that opened it, one client holds at most
//! half the slots, and a client that ended gives its streams back.

use crate::server::streams::{StreamTable, E_INVAL, E_OK, MAX_STREAMS, PER_OWNER};

const A: u32 = 30;
const B: u32 = 31;

/*
 * Ids are sequential, so any client could feed, pause or close another's
 * stream by guessing its id.
 */
#[test]
fn another_client_cannot_feed_pause_or_close_a_stream() {
    let mut t = StreamTable::new();
    let id = t.open(A).expect("a free slot");
    assert_eq!(t.feed(id, B, &[1, 2]), E_INVAL, "B cannot feed A's stream");
    assert!(!t.set_paused(id, B, true), "B cannot pause it");
    assert!(!t.close(id, B), "B cannot close it");
    assert_eq!(t.feed(id, A, &[1, 2]), E_OK, "A still can");
    assert!(t.close(id, A));
}

#[test]
fn a_client_stops_at_half_the_slots_and_another_still_opens() {
    let mut t = StreamTable::new();
    for _ in 0..PER_OWNER {
        assert!(t.open(A).is_some());
    }
    assert!(t.open(A).is_none(), "past its half");
    assert!(t.open(B).is_some(), "another client still opens");
}

/*
 * A client that ended kept its slots for good: four crashed players and no
 * program could open a stream again.
 */
#[test]
fn an_ended_client_s_streams_are_closed_and_a_living_one_s_kept() {
    let mut t = StreamTable::new();
    let a = t.open(A).expect("a slot");
    let b = t.open(B).expect("a slot");
    assert_eq!(t.reap(|pid| pid != A), 1);
    assert_eq!(t.feed(a, A, &[1]), E_INVAL, "A's stream is gone");
    assert_eq!(t.feed(b, B, &[1]), E_OK, "B's is kept");
    assert_eq!(t.reap(|_| true), 0);
}

#[test]
#[allow(clippy::assertions_on_constants)] // the relation is the point
fn one_client_never_holds_every_slot() {
    assert!(PER_OWNER < MAX_STREAMS);
}
