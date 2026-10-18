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

//! The report bounds: no input report goes unnumbered beside numbered
//! ones, a feature report past its bound costs only the feature fields,
//! and every item size reads alike.

use super::bounds_tests::XY;
use super::descs::{touch_pad, TIP};
use crate::hid::parse_report_descriptor;

#[test]
fn input_fields_ahead_of_the_first_report_id_are_refused() {
    let head: &[u8] = &[0x05, 0x0D, 0x09, 0x05, 0xA1, 0x01, 0x75, 0x08, 0x95, 0x01, 0x81, 0x02];
    let d = [head, &[0x85, 0x01], TIP, XY, &[0xC0]].concat();
    assert!(!parse_report_descriptor(&d).is_absolute_touch());
}

#[test]
fn a_feature_report_past_its_bound_costs_the_feature_fields_alone() {
    // Input Mode in feature report 2, then 600 fields of 256 bits, 150 KiB,
    // in feature report 3: past Linux's 16 KiB report.
    let mode: &[u8] = &[0x85, 0x02, 0x05, 0x0D, 0x09, 0x52, 0x75, 0x08, 0x95, 0x01, 0xB1, 0x02];
    let huge: &[u8] = &[0x85, 0x03, 0x76, 0x00, 0x01, 0x96, 0x58, 0x02, 0xB1, 0x03];
    let mode_only = parse_report_descriptor(&touch_pad(&[TIP, XY, mode]));
    assert!(mode_only.input_mode.present());
    let l = parse_report_descriptor(&touch_pad(&[TIP, XY, huge, mode]));
    assert!(l.is_absolute_touch(), "an oversized feature report refused the touch report");
    assert!(!l.input_mode.present(), "a feature field laid out past a refused feature report");
}

#[test]
fn every_data_size_of_an_item_reads_the_same_value() {
    // Report Count 1 and Logical Maximum 1200 in one, two and four bytes.
    for (count, max) in [
        (&[0x95, 0x01][..], &[0x26, 0xB0, 0x04][..]),
        (&[0x96, 0x01, 0x00][..], &[0x27, 0xB0, 0x04, 0x00, 0x00][..]),
        (&[0x97, 0x01, 0x00, 0x00, 0x00][..], &[0x26, 0xB0, 0x04][..]),
    ] {
        let xy = [
            &[0x05, 0x01, 0x75, 0x10][..],
            count,
            max,
            &[0x09, 0x30, 0x81, 0x02, 0x09, 0x31, 0x81, 0x02],
        ];
        let l = parse_report_descriptor(&touch_pad(&[TIP, &xy.concat()]));
        assert_eq!((l.x.bit_offset, l.y.bit_offset, l.x.logical_max), (8, 24, 1200));
    }
}
