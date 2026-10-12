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

//! A device that answers out of turn: an element naming a descriptor it was
//! never given, an index that moves by more than the request in flight. The
//! queue stalls for a reset, and nothing the device wrote is read.

use super::ring::Ring;
use crate::queue::Fault;

#[test]
fn an_element_naming_another_descriptor_stalls_the_queue() {
    for id in [1u32, 7, 8, u32::MAX] {
        let ring = Ring::new();
        let mut q = ring.queue();
        q.post().expect("posted");
        ring.complete(1, id, 64, 0xEE);
        assert_eq!(q.completion(), Some(Err(Fault::Foreign)), "id {id}");
        assert!(q.stalled(), "id {id}");
        assert_eq!(q.post(), Err(Fault::Stalled), "id {id}: nothing is posted over it");
        assert_eq!(ring.avail(None), 1, "id {id}");
    }
}

#[test]
fn an_index_that_moves_by_more_than_the_request_in_flight_stalls_the_queue() {
    for used in [2u16, 3, 0x8000, u16::MAX] {
        let ring = Ring::new();
        let mut q = ring.queue();
        q.post().expect("posted");
        ring.complete(used, 0, 64, 0xEE);
        assert_eq!(q.completion(), Some(Err(Fault::Foreign)), "used {used}");
        assert!(q.stalled(), "used {used}");
    }
}

#[test]
fn an_index_that_moves_with_nothing_in_flight_stalls_the_queue() {
    let ring = Ring::new();
    let mut q = ring.queue();
    ring.complete(1, 0, 64, 0xEE);
    assert_eq!(q.completion(), Some(Err(Fault::Foreign)));
    assert_eq!(q.post(), Err(Fault::Stalled));
}

#[test]
fn a_completion_that_wrote_nothing_hands_on_zeros_never_older_bytes() {
    // The buffer is emptied before each post: a device that reports bytes it
    // did not write cannot make the previous request's bytes go out again.
    let ring = Ring::new();
    let mut q = ring.queue();
    let mut out = [0u8; 64];
    q.post().expect("posted");
    ring.complete(1, 0, 64, 0xA1);
    let len = q.completion().expect("done").expect("ok");
    assert_eq!((q.take(len, &mut out), out), (64, [0xA1; 64]));
    q.post().expect("posted");
    ring.used(2, 0, 64);
    let len = q.completion().expect("done").expect("ok");
    assert_eq!((q.take(len, &mut out), out), (64, [0; 64]));
}
