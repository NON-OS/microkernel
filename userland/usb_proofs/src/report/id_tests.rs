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

//! Report IDs and logical ranges as a device states them: a report is read
//! only by the parts of its own ID, so a media key report never types a
//! letter, and a Logical Maximum is signed when its minimum is (HID 1.11,
//! 6.2.2.7), unsigned when not, as devices write 0xFF for 255.

use super::build::app;
use super::layout;
use crate::hid::{Device, ReportDevice};
use nonos_libc::{reset, take_posted};

/// A keyboard under report ID 1 (modifier bits, one key slot) and consumer
/// control under ID 2 (one 16-bit usage slot).
const KEYS_AND_MEDIA: &[u8] = &[
    0x05, 0x01, 0x09, 0x06, 0xa1, 0x01, 0x85, 0x01, 0x05, 0x07, 0x19, 0xe0, 0x29, 0xe7, 0x15, 0x00,
    0x25, 0x01, 0x75, 0x01, 0x95, 0x08, 0x81, 0x02, 0x95, 0x01, 0x75, 0x08, 0x25, 0x65, 0x19, 0x00,
    0x29, 0x65, 0x81, 0x00, 0xc0, 0x05, 0x0c, 0x09, 0x01, 0xa1, 0x01, 0x85, 0x02, 0x19, 0x00, 0x2a,
    0x3c, 0x02, 0x15, 0x00, 0x26, 0x3c, 0x02, 0x95, 0x01, 0x75, 0x10, 0x81, 0x00, 0xc0,
];

fn posted_codes(dev: &mut Device, report: &[u8]) -> Vec<(u16, u32)> {
    dev.feed(report);
    take_posted().iter().map(|e| (e.kind, e.code)).collect()
}

#[test]
fn a_report_is_read_only_by_the_parts_of_its_own_id() {
    reset();
    let mut dev = Device::Report(Box::new(ReportDevice::new(layout(KEYS_AND_MEDIA))));
    // As a keyboard report these bytes would press A; under ID 2 they name
    // consumer usage 0x400, no key at all.
    assert_eq!(posted_codes(&mut dev, &[2, 0x00, 0x04]), vec![]);
    assert_eq!(posted_codes(&mut dev, &[1, 0x00, 0x04]), vec![(0, u32::from(b'a'))]);
    let volume_up = posted_codes(&mut dev, &[2, 0xe9, 0x00]);
    assert_eq!(volume_up.len(), 1, "one key, and A stays down: {volume_up:?}");
}

#[test]
fn a_logical_maximum_is_signed_only_under_a_negative_minimum() {
    // Absolute X and Y, 8 bits each, the minimum -128 or 0, the maximum 0xFF.
    let axes = |min: u8| -> Vec<u8> {
        let fields = [0x05, 0x01, 0x09, 0x30, 0x09, 0x31, 0x15, min, 0x25, 0xff];
        app(&[&fields, &[0x75, 0x08, 0x95, 0x02, 0x81, 0x02]])
    };
    let x = |d: &[u8]| layout(d).pointer.and_then(|p| p.x).map(|f| (f.min, f.max));
    assert_eq!(x(&axes(0x80)), Some((-128, -1)));
    assert_eq!(x(&axes(0x00)), Some((0, 255)));
}
