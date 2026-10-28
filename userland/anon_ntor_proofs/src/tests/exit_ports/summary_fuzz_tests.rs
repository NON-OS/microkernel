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

//! Bytes that are not a `p` line at all: the reader refuses them or keeps a
//! bounded, well formed summary, and never panics.

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::microdesc::parse;
use crate::directory::microdesc::policy::exits_web;
use crate::directory::microdesc::summary::{summary, RANGES_MAX};

#[test]
fn random_bytes_never_panic_the_line_or_the_microdescriptor() {
    let mut x = 0x9E37_79B9_7F4A_7C15u64;
    let alphabet = b"acceptrj 0123456789-,\n\xff";
    for round in 0..20_000usize {
        let len = round % 97;
        let bytes: Vec<u8> = (0..len)
            .map(|_| {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                if round & 1 == 0 {
                    alphabet[(x % alphabet.len() as u64) as usize]
                } else {
                    x as u8
                }
            })
            .collect();
        if let Some(s) = summary(&bytes) {
            assert!(s.ranges.len() <= RANGES_MAX);
            assert!(s.ranges.iter().all(|&(low, high)| 1 <= low && low <= high));
        }
        let _ = (exits_web(&bytes), parse(&bytes));
    }
}
