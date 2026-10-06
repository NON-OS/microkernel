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

//! A datagram socket's receive queue is this capsule's memory. Each
//! waiting datagram is charged as Linux charges it, its payload and an
//! sk_buff's overhead, so a queue of empty datagrams fills like any other
//! and no sender, however small its datagrams, grows one without end.

use super::random::Regs;
use crate::linux::net::sock::gram_room::{cost, takes, OVERHEAD};

/// Linux's SO_RCVBUF bounds, as setsockopt doubles them (net/opt/ids.rs).
const RCVBUF_MIN: u32 = 2304;
const RCVBUF_MAX: u32 = 2 * 212_992;
/// The largest UDP payload IPv4 carries (sock/gram_target.rs MAX_GRAM).
const MAX_GRAM: usize = 65507;

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

/// As on Linux, an empty queue takes one datagram larger than its buffer,
/// and nothing after it until that is read.
#[test]
fn an_empty_queue_takes_one_datagram_larger_than_its_buffer() {
    assert!(takes(0, RCVBUF_MIN));
    assert!(!takes(cost(MAX_GRAM), RCVBUF_MIN));
}

#[test]
fn a_queue_never_holds_more_than_its_buffer_and_one_datagram() {
    let mut r = Regs::new(0x0D06_0A11);
    for _ in 0..2_000 {
        let rcvbuf = RCVBUF_MIN + (r.small(u64::from(RCVBUF_MAX - RCVBUF_MIN)) as u32);
        let (mut queued, mut count) = (0usize, 0usize);
        for _ in 0..5_000 {
            let len = r.small(MAX_GRAM as u64 + 1) as usize;
            if takes(queued, rcvbuf) {
                queued += cost(len);
                count += 1;
            }
        }
        assert!(queued <= rcvbuf as usize + cost(MAX_GRAM), "{queued} in {rcvbuf}");
        assert!(count <= rcvbuf as usize / OVERHEAD + 1, "{count} in {rcvbuf}");
    }
}

#[test]
fn a_charge_is_the_payload_and_the_overhead() {
    assert_eq!(cost(0), OVERHEAD);
    assert_eq!(cost(1000), 1000 + OVERHEAD);
    assert_eq!(cost(usize::MAX), usize::MAX, "saturates");
}
