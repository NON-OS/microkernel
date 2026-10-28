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

//! Write, then close: every byte the guest wrote reaches the far end, in
//! order and once, whatever answers the proxy loses, and the close waits
//! for it no longer than its bound. The queue and the linger as they ship
//! (queue.rs, linger.rs), the steps as closing.rs and pump.rs take them.

use super::proxy::Proxy;
use crate::linux::net::routed::linger::{ends, linger, LINGER_MS};
use crate::linux::net::routed::queue::{Queue, TX_MAX};

pub(super) const TICK: i64 = 10;

/// One carry: the frame offered, and the proxy's answer to it, if any.
pub(super) fn carry(q: &mut Queue, p: &mut Proxy) {
    if let Some(frame) = q.offer().map(<[u8]>::to_vec) {
        if p.call(&frame) {
            q.taken();
        }
    }
}

/// The guest writes `data` in pieces as room allows, the stream carried
/// between writes, and closes right after its last write; then the close's
/// linger. What reached the far end, and when the stream ended.
fn write_then_close(p: &mut Proxy, data: &[u8]) -> i64 {
    let (mut q, mut at, mut now) = (Queue::default(), 0, 0i64);
    while at < data.len() {
        let piece = 1 + (p.next() % 3_000) as usize;
        at += q.push(&data[at..(at + piece).min(data.len())]);
        if at < data.len() {
            carry(&mut q, p);
            now += TICK;
        }
    }
    let closed = now;
    let until = linger(q.queued(), now).unwrap_or(now);
    while !ends(q.queued(), until, now) {
        carry(&mut q, p);
        now += TICK;
    }
    now - closed
}

#[test]
fn every_byte_written_before_a_close_goes_once_and_in_order() {
    for seed in 1..300u64 {
        let mut p = Proxy::new(seed, true);
        let len = (p.next() % (3 * TX_MAX as u64)) as usize + 1;
        let data: Vec<u8> = (0..len).map(|_| p.next() as u8).collect();
        let took = write_then_close(&mut p, &data);
        assert!(took <= LINGER_MS, "seed {seed}: the close waited {took} ms");
        assert_eq!(p.far.len(), data.len(), "seed {seed}");
        assert!(p.far == data, "seed {seed}: what went is what was written");
    }
}
