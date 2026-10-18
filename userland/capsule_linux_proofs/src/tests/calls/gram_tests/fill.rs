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

//! Empty datagrams fill a queue like any other, and it stops.

use crate::linux::net::sock::gram_room::{cost, takes, OVERHEAD};

/// Linux's SO_RCVBUF bounds, as setsockopt doubles them (net/opt/ids.rs).
pub(super) const RCVBUF_MIN: u32 = 2304;
pub(super) const RCVBUF_MAX: u32 = 2 * 212_992;

/// The largest UDP payload IPv4 carries (sock/gram_target.rs MAX_GRAM).
pub(super) const MAX_GRAM: usize = 65507;

/// Queue datagrams of `len` bytes until the receiver stops taking them.
fn fill(rcvbuf: u32, len: usize) -> (usize, usize) {
    let (mut queued, mut count) = (0usize, 0usize);
    while takes(queued, rcvbuf) {
        queued += cost(len);
        count += 1;
        assert!(count <= rcvbuf as usize + 1, "never refused");
    }
    (count, queued)
}

/// The guest that grew the queue: empty datagrams to its own socket, in a
/// loop. They stop where Linux's do, a few hundred deep.
#[test]
fn empty_datagrams_fill_the_queue_and_stop() {
    let (count, _) = fill(212_992, 0);
    assert_eq!(count, 212_992 / OVERHEAD + 1);
    let (count, _) = fill(RCVBUF_MAX, 0);
    assert!(count <= RCVBUF_MAX as usize / OVERHEAD + 1, "{count}");
    let (count, _) = fill(RCVBUF_MIN, 0);
    assert_eq!(count, RCVBUF_MIN as usize / OVERHEAD + 1);
}
