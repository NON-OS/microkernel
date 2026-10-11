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

use super::error::WireError;
use super::lba_map::{Partial, Whole};
use super::span::span;

#[test]
fn at_512_byte_blocks_sectors_pass_straight_through() {
    let p = span(512, 100, 10, 4 * 512).unwrap();
    assert_eq!(p.head, None);
    assert_eq!(p.tail, None);
    assert_eq!(p.body, Some(Whole { lba: 10, lbas: 4, at: 0 }));
}

#[test]
fn a_4k_disk_takes_unaligned_ends_as_partial_blocks() {
    // Sectors 7..=16: the last sector of block 0, all of block 1, the
    // first sector of block 2.
    let p = span(4096, 4, 7, 10 * 512).unwrap();
    assert_eq!(p.head, Some(Partial { lba: 0, offset: 3584, len: 512, at: 0 }));
    assert_eq!(p.body, Some(Whole { lba: 1, lbas: 1, at: 512 }));
    assert_eq!(p.tail, Some(Partial { lba: 2, offset: 0, len: 512, at: 4608 }));
}

#[test]
fn the_last_block_is_reachable_and_one_past_it_is_not() {
    assert!(span(512, 100, 99, 512).is_ok());
    assert_eq!(span(512, 100, 100, 512), Err(WireError::OutOfRange));
    assert_eq!(span(512, 100, 98, 3 * 512), Err(WireError::OutOfRange));
    // A part of a 4K block past the end is refused too, not read short.
    assert!(span(4096, 2, 15, 512).is_ok());
    assert_eq!(span(4096, 2, 16, 512), Err(WireError::OutOfRange));
}

#[test]
fn empty_ragged_and_overflowing_requests_are_refused() {
    assert_eq!(span(512, 100, 0, 0), Err(WireError::InvalidArgument));
    assert_eq!(span(512, 100, 0, 511), Err(WireError::InvalidArgument));
    assert_eq!(span(512, u64::MAX, u64::MAX, 512), Err(WireError::OutOfRange));
    assert_eq!(span(3000, 100, 0, 512), Err(WireError::OutOfRange));
}
