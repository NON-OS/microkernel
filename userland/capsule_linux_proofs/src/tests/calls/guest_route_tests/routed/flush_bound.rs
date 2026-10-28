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

//! The close's bound, and what ends a stream at its close: a proxy that
//! never answers is given up on at LINGER_MS with nothing carried twice, and
//! a stream with nothing queued, or whose write failed, ends at once.

use super::flush::{carry, TICK};
use super::proxy::Proxy;
use crate::linux::net::routed::linger::{ends, linger, LINGER_MS};
use crate::linux::net::routed::queue::{Queue, FRAME_MAX, TX_MAX};

#[test]
fn a_proxy_that_never_answers_ends_the_stream_at_its_bound_with_nothing_doubled() {
    let mut p = Proxy::new(7, false);
    let mut q = Queue::default();
    assert_eq!(q.push(&[1u8; 100]), 100);
    let until = linger(q.queued(), 0).expect("written bytes keep the stream");
    let mut now = 0;
    while !ends(q.queued(), until, now) {
        carry(&mut q, &mut p);
        now += TICK;
    }
    assert_eq!(now, LINGER_MS);
    assert!(p.far.len() <= 100, "carried at most once");
}

#[test]
fn a_stream_with_nothing_queued_or_a_failed_write_ends_at_the_close() {
    let mut q = Queue::default();
    assert_eq!(linger(q.queued(), 5), None);
    q.push(b"GET / ");
    assert_eq!(q.offer(), Some(&b"GET / "[..]));
    q.push(b"HTTP/1.1\r\n\r\n");
    assert_eq!(q.offer(), Some(&b"GET / "[..]), "a frame out goes again unchanged");
    assert!(q.out());
    q.fail();
    assert!(!q.queued() && q.broken() && q.room() == 0 && q.offer().is_none());
    assert_eq!((q.push(b"more"), linger(q.queued(), 5)), (0, None));
    assert!(ends(false, i64::MAX, 0));
    assert_eq!((FRAME_MAX, TX_MAX), (crate::frame::CARRY_MAX, 4 * FRAME_MAX));
}
