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

//! Whole messages handed on once and in order, refused ones walked past.

use crate::wire::{walk, HEADER};

/// A message to object 1 with opcode `op` and `extra` bytes of arguments.
pub(super) fn message(op: u16, extra: usize) -> Vec<u8> {
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
