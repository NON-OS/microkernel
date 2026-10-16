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

//! Logical extents and usages as items give them (HID 1.11, 6.2.2.7 and
//! 6.2.2.8).

use super::maps::{global, item, local, main};
use nonos_bt_hogp::report::map::{parse, MapError, Span};

#[test]
fn a_one_octet_maximum_of_255_is_unsigned_and_minus_one_stays_signed() {
    let field = |min: u8, max: u8| {
        let m = [global(0, 7), local(1, 0), local(2, 0xff), global(1, min), global(2, max)];
        [m.concat(), global(7, 8), global(9, 1), main(8, 0)].concat()
    };
    let l = parse(&field(0, 0xff)).unwrap();
    assert_eq!(l.fields[0].lmax, 255);
    let l = parse(&field(0xff, 0x01)).unwrap();
    assert_eq!((l.fields[0].lmin, l.fields[0].lmax), (-1, 1));
}

#[test]
fn a_usage_range_stays_on_one_page() {
    let min = item(2, 1, &[0x04, 0x00, 0x07, 0x00]);
    let m = [global(0, 0x0c), min, local(2, 0x10)].concat();
    assert_eq!(parse(&m), Err(MapError::Value), "a Minimum on page 7, a Maximum on page 0x0C");
}

#[test]
fn a_four_octet_usage_names_its_own_page() {
    let usage = item(2, 0, &[0xe9, 0x00, 0x0c, 0x00]);
    let m = [global(0, 7), usage, global(7, 1), global(9, 1), global(2, 1), main(8, 2)].concat();
    assert_eq!(parse(&m).unwrap().fields[0].spans, [Span { page: 0x0c, min: 0xe9, max: 0xe9 }]);
}
