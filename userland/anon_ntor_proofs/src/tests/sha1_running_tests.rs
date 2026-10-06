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

//! The two running-hash properties the relay digest chain needs.

extern crate alloc;

use alloc::vec::Vec;

use crate::sha1::Sha1;

/// One cell's worth of payload, which is deliberately not a multiple of the 64
const CELL: usize = 509;

fn pattern(len: u32) -> Vec<u8> {
    (0..len).map(|i| (i % 251) as u8).collect()
}

/*
 * A relay digest absorbs one 509 byte payload at a time. If buffering across
 * calls were wrong the first cell would still verify and the second would not,
 * which on a live circuit looks like a network fault rather than a bug.
 */
#[test]
fn chunked_updates_equal_one_update() {
    let data = pattern(2_000);
    let mut whole = Sha1::new();
    whole.update(&data);
    let mut chunked = Sha1::new();
    for chunk in data.chunks(CELL) {
        chunked.update(chunk);
    }
    assert_eq!(whole.finish(), chunked.finish());
}

#[test]
fn peek_does_not_end_the_hash() {
    let data = pattern(1_200);
    let mut whole = Sha1::new();
    whole.update(&data);
    let mut peeked = Sha1::new();
    peeked.update(&data[..CELL]);
    let first = peeked.peek();
    assert_eq!(first, peeked.peek(), "peek is not idempotent");
    peeked.update(&data[CELL..]);
    assert_eq!(whole.finish(), peeked.finish());
}
