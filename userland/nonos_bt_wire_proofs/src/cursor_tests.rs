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

//! The reader takes little-endian fields (Vol 4, Part E, 5.2) and refuses,
//! never panics, past the end.

use nonos_bt_wire::cursor::{Reader, Writer};
use nonos_bt_wire::WireError;

#[test]
fn fields_are_little_endian_both_ways() {
    let mut w = Writer::new();
    w.u8(0x01).u16(0x0c03).bytes(&[9, 8]);
    assert_eq!(w.into_vec(), [0x01, 0x03, 0x0c, 9, 8]);
    let mut r = Reader::new(&[0x01, 0x03, 0x0c, 9, 8]);
    assert_eq!((r.u8(), r.u16()), (Ok(1), Ok(0x0c03)));
    assert_eq!(r.array::<2>(), Ok([9, 8]));
    assert_eq!(r.finish(), Ok(()));
}

#[test]
fn reading_past_the_end_is_short_and_leftovers_are_length() {
    let mut r = Reader::new(&[1]);
    assert_eq!(r.u16(), Err(WireError::Short));
    assert_eq!(r.bytes(usize::MAX), Err(WireError::Short));
    assert_eq!(r.remaining(), 1);
    assert_eq!(r.finish(), Err(WireError::Length));
}
