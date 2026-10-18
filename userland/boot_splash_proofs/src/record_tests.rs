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

//! The 28-byte record: every field where F-50 puts it, a short buffer and a
//! turn count past three refused.

use crate::logo::record::{Record, ON_SCREEN, RECORD_LEN, TOP_DOWN};

#[test]
fn every_field_is_read_where_the_note_puts_it() {
    let mut b = [0u8; RECORD_LEN];
    for (o, v) in [(0, 10u32), (4, 20), (8, 300), (12, 200), (20, 180_054), (24, 54)] {
        b[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
    b[16] = ON_SCREEN | TOP_DOWN;
    b[17] = 3;
    b[18..20].copy_from_slice(&24u16.to_le_bytes());
    let r = Record::parse(&b).unwrap();
    assert_eq!((r.x, r.y, r.width, r.height), (10, 20, 300, 200));
    assert_eq!((r.flags, r.turns, r.bpp, r.file_size, r.pixel_offset), (3, 3, 24, 180_054, 54));
}

#[test]
fn short_records_and_a_fourth_turn_are_refused() {
    assert!(Record::parse(&[0u8; RECORD_LEN - 1]).is_none());
    let mut b = [0u8; RECORD_LEN];
    b[17] = 4;
    assert!(Record::parse(&b).is_none());
    b[17] = 0;
    assert!(Record::parse(&b).is_some());
}
