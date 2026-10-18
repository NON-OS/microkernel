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

//! Fields that are not what a key needs: array values past the logical
//! extent though the usages go on, wide variable fields, and padding at
//! the end of a report that a short report leaves out.

use super::maps::{bits_field, global, local, main};
use nonos_bt_hogp::report::map::{held, parse};

#[test]
fn an_array_value_past_the_logical_maximum_holds_nothing() {
    // Usages run to 0xFF, the extent only to 0x65.
    let m = [global(0, 7), local(1, 0), local(2, 0xff), global(1, 0), global(2, 0x65)];
    let l = parse(&[m.concat(), global(7, 8), global(9, 1), main(8, 0)].concat()).unwrap();
    assert_eq!(held(&l, 0, &[0x65]), Some(vec![0x65]));
    assert_eq!(held(&l, 0, &[0x70]), Some(vec![]));
}

#[test]
fn a_wide_variable_field_is_a_quantity_not_a_key() {
    let m = [global(0, 7), local(0, 0x04), global(1, 0), global(2, 0x7f), global(7, 8)];
    let l = parse(&[m.concat(), global(9, 1), main(8, 2)].concat()).unwrap();
    assert_eq!(held(&l, 0, &[0x05]), Some(vec![]));
}

#[test]
fn a_report_short_of_its_padding_changes_nothing() {
    let m = [bits_field(0, 0xe0, 8), global(7, 8), global(9, 1), main(8, 1)].concat();
    let l = parse(&m).unwrap();
    assert_eq!(held(&l, 0, &[0x01, 0x00]), Some(vec![0xe0]));
    assert_eq!(held(&l, 0, &[0x01]), None);
}

#[test]
fn a_signed_array_counts_from_its_minimum() {
    // Logical -2 to 1 over usages 0x04 to 0x07, in four-bit elements.
    let m = [global(0, 7), local(1, 4), local(2, 7), global(1, 0xfe), global(2, 1)];
    let m = [m.concat(), global(7, 4), global(9, 2), main(8, 0)].concat();
    let l = parse(&m).unwrap();
    assert_eq!(held(&l, 0, &[0x2e]), Some(vec![0x04]), "-2 is the first usage, 2 none");
    assert_eq!(held(&l, 0, &[0x1f]), Some(vec![0x05, 0x07]), "-1, then 1");
    assert_eq!(held(&l, 0, &[0x82]), Some(vec![]), "2 and -8 lie outside");
}
