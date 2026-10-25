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

//! The reader every parser stands on: reads stop at the end, lengths are
//! judged against their caps before the bytes, and a body must be used up.

use nsh_wire::{Reader, WireError};

#[test]
fn a_read_past_the_end_is_short_and_moves_nothing() {
    let mut r = Reader::new(&[1, 2, 3]);
    assert_eq!(r.u32(), Err(WireError::Short));
    assert_eq!(r.left(), 3);
    assert_eq!(r.take(usize::MAX), Err(WireError::Short));
    assert_eq!(r.take(3), Ok(&[1, 2, 3][..]));
    assert_eq!(r.finish(), Ok(()));
}

#[test]
fn integers_are_little_endian() {
    let mut r = Reader::new(&[1, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!((r.u16(), r.u32(), r.u64()), (Ok(1), Ok(2), Ok(3)));
}

#[test]
fn a_length_over_its_cap_is_oversize_even_with_no_bytes_behind_it() {
    assert_eq!(Reader::new(&[9]).bytes8(1, 8), Err(WireError::Oversize));
    assert_eq!(Reader::new(&[9, 0]).bytes16(1, 8), Err(WireError::Oversize));
    assert_eq!(Reader::new(&[0xFF; 4]).bytes32(1, 8), Err(WireError::Oversize));
}

#[test]
fn a_length_under_its_floor_is_a_bad_value() {
    assert_eq!(Reader::new(&[0]).bytes8(1, 8), Err(WireError::Value));
    assert_eq!(Reader::new(&[1, 0, 0xAA]).bytes16(2, 8), Err(WireError::Value));
}

#[test]
fn a_length_at_its_cap_with_the_bytes_reads_them() {
    let mut r = Reader::new(&[2, 7, 8]);
    assert_eq!(r.bytes8(1, 2), Ok(&[7, 8][..]));
    assert_eq!(r.finish(), Ok(()));
}

#[test]
fn bytes_left_over_are_refused() {
    let mut r = Reader::new(&[1, 2]);
    assert_eq!(r.u8(), Ok(1));
    assert_eq!(r.finish(), Err(WireError::Trailing));
}
