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

//! The display server walks a batch of requests where it lies and cuts
//! what it served from the queue once. Walking takes the queue by shared
//! reference, so nothing in it can cut the queue a message at a time, which
//! moved every byte behind each message. Held here: every whole message is
//! handed on once and in order, a refused one is still walked past, and the
//! walk stops at the first message not yet whole.

use super::random::Regs;
use crate::wire::{next, walk, HEADER};

/// A message to object 1 with opcode `op` and `extra` bytes of arguments.
fn message(op: u16, extra: usize) -> Vec<u8> {
    let size = (HEADER + extra) as u16;
    let mut m = Vec::new();
    m.extend_from_slice(&1u32.to_le_bytes());
    m.extend_from_slice(&op.to_le_bytes());
    m.extend_from_slice(&size.to_le_bytes());
    m.resize(HEADER + extra, 0);
    m
}

#[test]
fn every_whole_message_is_handed_on_once_and_in_order() {
    let mut buf = Vec::new();
    for op in 0..100u16 {
        buf.extend(message(op, (op as usize % 5) * 4));
    }
    let whole = buf.len();
    buf.extend(&message(7, 40)[..20]);
    let mut seen = Vec::new();
    assert_eq!(
        walk(&buf, |m| {
            seen.push(m.opcode);
            true
        }),
        whole,
        "not the partial one"
    );
    assert_eq!(seen, (0..100).collect::<Vec<u16>>());
}

#[test]
fn a_refused_message_is_walked_past_and_the_walk_stops() {
    let mut buf = Vec::new();
    for op in 0..10u16 {
        buf.extend(message(op, 4));
    }
    let mut seen = 0;
    let walked = walk(&buf, |_| {
        seen += 1;
        seen < 3
    });
    assert_eq!((seen, walked), (3, 36), "the third is walked, so it is never handed on again");
    let mut again = Vec::new();
    walk(&buf[walked..], |m| {
        again.push(m.opcode);
        true
    });
    assert_eq!(again, (3..10).collect::<Vec<u16>>());
}

#[test]
fn a_header_shorter_than_a_header_is_never_walked() {
    let mut buf = message(1, 0);
    buf[6] = 4;
    assert_eq!(walk(&buf, |_| true), 0);
    assert_eq!(walk(&[0u8; 7], |_| true), 0);
}

/// The batch that took tens of seconds: a mebibyte of 12-byte syncs in one
/// write, walked in one pass.
#[test]
fn a_mebibyte_of_small_requests_is_walked_once() {
    let one = message(0, 4);
    let buf: Vec<u8> = one.iter().copied().cycle().take(one.len() * 87_381).collect();
    let mut count = 0;
    assert_eq!(
        walk(&buf, |_| {
            count += 1;
            true
        }),
        buf.len()
    );
    assert_eq!(count, 87_381);
}

/// Over any bytes, the walk covers exactly the whole messages at the front.
#[test]
fn the_walk_is_the_whole_messages_at_the_front() {
    let mut r = Regs::new(0x0A11_C0DE);
    for _ in 0..20_000 {
        let len = r.small(200) as usize;
        let buf: Vec<u8> = (0..len).map(|_| r.any() as u8 % 24).collect();
        let mut want = 0;
        while let Some((_, size)) = next(&buf[want..]) {
            want += size;
        }
        assert_eq!(walk(&buf, |_| true), want);
    }
}
