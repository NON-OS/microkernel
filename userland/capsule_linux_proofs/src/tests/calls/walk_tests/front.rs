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

//! Short headers, a mebibyte of requests, and the walk's front.

use super::super::super::random::Regs;
use super::order::message;
use crate::wire::{next, walk};

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
