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

//! A pad that also declares an outsized vendor report, as Linux drives one:
//! a 4 KiB feature report (a firmware-update blob) or a 300-byte input
//! report (raw sensor data). The pad is driven and that report is ignored.

use nonos_i2cmodel::touchpad::{report_descriptor, touch_report, Touch};
use nonos_i2cmodel::HidOverI2c;
use nonos_libc::{take_events, INPUT_KIND_POINTER_REL};

use super::fixture::{of_kind, rig_with, HID_DESC_REG, PAD};
use crate::input::poll;
use crate::setup;

/// Vendor application 0xFF00:1, report 0x0E: 0x1000 feature bytes.
const BLOB: &[u8] = &[
    0x06, 0x00, 0xFF, 0x09, 0x01, 0xA1, 0x01, 0x85, 0x0E, 0x09, 0x02, 0x15, 0x00, 0x26, 0xFF, 0x00,
    0x75, 0x08, 0x96, 0x00, 0x10, 0xB1, 0x02, 0xC0,
];
/// Vendor application 0xFF00:1, report 0x0D: 300 input bytes.
const RAW: &[u8] = &[
    0x06, 0x00, 0xFF, 0x09, 0x01, 0xA1, 0x01, 0x85, 0x0D, 0x09, 0x02, 0x15, 0x00, 0x26, 0xFF, 0x00,
    0x75, 0x08, 0x96, 0x2C, 0x01, 0x81, 0x02, 0xC0,
];

/// The bench pad with `vendor` after it, bound, sent `junk` first; and
/// whether a finger that slides on it moves the pointer.
fn moves_with(vendor: &[u8], max_input: u16, junk: &[u8]) -> bool {
    let desc = [report_descriptor(), vendor.to_vec()].concat();
    let r =
        rig_with(HidOverI2c::new(PAD, desc, max_input, 0x04F3, 0x3028), Some((PAD, HID_DESC_REG)));
    let mut s = setup::run().expect("setup");
    assert!(s.found() && !s.done, "the pad was not driven");
    take_events();
    if !junk.is_empty() {
        r.pad.lock().push_input(junk);
        poll(&mut s);
        assert!(take_events().is_empty(), "the vendor report was read as input");
    }
    for x in [100, 140] {
        r.pad.lock().push_input(&touch_report(&Touch::finger(x, 100)));
        poll(&mut s);
    }
    !of_kind(&take_events(), INPUT_KIND_POINTER_REL).is_empty()
}

#[test]
fn a_4_kib_vendor_feature_report_does_not_keep_the_pad_from_being_driven() {
    assert!(moves_with(BLOB, 2 + 8, &[]), "a finger did not move the pointer");
}

#[test]
fn a_300_byte_vendor_input_report_is_ignored_and_the_pad_driven() {
    let raw = [vec![0x0D], vec![0x5A; 300]].concat();
    assert!(moves_with(RAW, 2 + 301, &raw), "a finger did not move the pointer");
}
