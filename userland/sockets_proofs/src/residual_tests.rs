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

//! A read on a mixnet socket pulls a frame off the queue only when what it
//! cannot take can be kept: the table of remainders is shared by every
//! process, so no one process may hold bytes in every slot, and a read
//! that could not keep its rest must not lose it. One test, since the table
//! is one static the steps share.

use crate::server::handlers::mixnet_frame::MAX_BODY;
use crate::server::handlers::mixnet_residual::{release, room, store, take};
use crate::sockets::SocketKey;

fn key(pid: u32, handle: u32) -> SocketKey {
    SocketKey { pid, handle }
}

/// Leave a remainder on `pid`'s sockets until a read is refused: how many.
fn fill(pid: u32) -> u32 {
    let mut held = 0;
    while room(key(pid, held), 1) {
        assert!(store(key(pid, held), &[1, 2, 3]));
        held += 1;
        assert!(held < 4096, "the table has a bound");
    }
    held
}

#[test]
fn no_process_takes_every_slot_and_a_full_table_refuses_before_a_frame_is_pulled() {
    let (hog, second, other) = (7, 8, key(9, 1));
    // Empty: any read may pull a frame.
    assert!(room(other, 1));
    // One process is stopped at its share, and another may still keep bytes.
    let share = fill(hog);
    assert!(share > 1);
    assert!(!room(key(hog, share), 1));
    assert!(!store(key(hog, share), &[1]));
    assert!(room(other, 1));
    // A second process takes the same share and the table is full: half each.
    assert_eq!(fill(second), share);
    // A third's short and zero-length reads are refused before a frame is
    // pulled; a buffer that takes any body whole needs no slot and is not.
    assert!(!room(other, 1));
    assert!(!room(other, 0));
    assert!(!room(other, MAX_BODY - 1));
    assert!(room(other, MAX_BODY));
    // room and store agree: what room refuses, store could not have kept.
    assert!(!store(other, &[4]));
    // A socket with bytes still held may pull no frame, even a zero-length
    // read, which take answers with nothing.
    let mut none = [0u8; 0];
    assert_eq!(take(key(hog, 0), &mut none), 0);
    assert!(!room(key(hog, 0), 0));
    // Drained, its slot is free again.
    let mut out = [0u8; 3];
    assert_eq!(take(key(hog, 0), &mut out), 3);
    assert_eq!(out, [1, 2, 3]);
    assert!(room(key(hog, 0), 1));
    assert!(room(other, 1));
    // A closed socket's slot is free to another process.
    release(key(second, 1));
    assert!(store(other, &[5, 6]));
    let mut got = [0u8; 2];
    assert_eq!(take(other, &mut got), 2);
    assert_eq!(got, [5, 6]);
    // Every slot let go, then one process refills the drained slots its
    // sockets kept: still held to its share.
    for h in 0..share {
        release(key(hog, h));
        release(key(second, h));
    }
    release(other);
    let (p, q, kept) = (20, 21, 100);
    assert_eq!(fill(q), share);
    for h in kept..kept + share {
        assert!(room(key(p, h), 1));
        assert!(store(key(p, h), &[7]));
        assert_eq!(take(key(p, h), &mut out), 1);
    }
    for h in 0..share {
        assert_eq!(take(key(q, h), &mut out), 3);
    }
    assert_eq!(fill(p), share);
    for h in kept..kept + share {
        assert!(!room(key(p, h), 1), "a kept drained slot past the share");
        assert!(!store(key(p, h), &[8]));
    }
    assert!(room(other, 1), "another process still keeps bytes");
}
