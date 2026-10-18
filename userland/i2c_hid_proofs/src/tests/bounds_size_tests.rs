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

//! The bounds on a global item's value: Report ID 0 is reserved, a Report
//! Size past 256 bits refuses the descriptor, and a field wider than 32
//! bits is laid out but never read.

use super::bounds_tests::lays_out;
use super::descs::{touch_pad, TIP};
use crate::hid::parse_report_descriptor;

#[test]
fn report_id_zero_is_reserved_and_refused() {
    assert!(!lays_out(&[0x85, 0x00]));
    assert!(lays_out(&[0x85, 0xFF, 0x75, 0x08, 0x95, 0x01, 0x81, 0x03]), "report ID 255");
}

#[test]
fn a_report_size_past_256_bits_is_refused_and_a_field_past_32_is_not_read() {
    assert!(lays_out(&[0x76, 0x00, 0x01, 0x95, 0x01, 0x81, 0x03]), "256 constant bits");
    assert!(!lays_out(&[0x76, 0x01, 0x01, 0x95, 0x01, 0x81, 0x03]), "257 bits");
    let x = |bits: u8| [0x05, 0x01, 0x75, bits, 0x95, 0x01, 0x09, 0x30, 0x81, 0x02];
    let y: &[u8] = &[0x75, 0x10, 0x09, 0x31, 0x81, 0x02];
    let with_x = |bits| parse_report_descriptor(&touch_pad(&[TIP, &x(bits), y]));
    assert_eq!(with_x(32).x.bit_size, 32, "a 32-bit X is read");
    assert!(!with_x(33).is_absolute_touch(), "a 33-bit X was taken");
}
