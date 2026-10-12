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
//! Multitouch report descriptors built an item at a time (HID 1.11 short
//! items; Digitizers page 0Dh, as recalled, unverified): `fingers` finger
//! collections of Tip Switch, Confidence, six pad bits, an 8-bit Contact
//! Identifier and 16-bit X (to 4000) and Y (to 3000), then Contact Count
//! and one button, all in report 4, under an Application collection of
//! `kind` (04h a touchscreen, 05h a touchpad). A report is 1 + 6 a finger
//! + 2 bytes.

pub const SCREEN: u8 = 0x04;
pub const PAD: u8 = 0x05;
pub const RID: u8 = 4;

fn finger() -> Vec<u8> {
    let mut d = vec![0x05, 0x0D, 0x09, 0x22, 0xA1, 0x02];
    d.extend([0x09, 0x42, 0x09, 0x47, 0x25, 0x01, 0x75, 0x01, 0x95, 0x02, 0x81, 0x02]);
    d.extend([0x95, 0x06, 0x81, 0x03]);
    d.extend([0x09, 0x51, 0x25, 0x7F, 0x75, 0x08, 0x95, 0x01, 0x81, 0x02]);
    d.extend([0x05, 0x01, 0x26, 0xA0, 0x0F, 0x75, 0x10, 0x09, 0x30, 0x81, 0x02]);
    d.extend([0x26, 0xB8, 0x0B, 0x09, 0x31, 0x81, 0x02, 0xC0]);
    d
}

pub fn digitizer(kind: u8, fingers: usize) -> Vec<u8> {
    let mut d = vec![0x05, 0x0D, 0x09, kind, 0xA1, 0x01, 0x85, RID];
    (0..fingers).for_each(|_| d.extend(finger()));
    d.extend([0x05, 0x0D, 0x09, 0x54, 0x25, 0x7F, 0x75, 0x08, 0x95, 0x01, 0x81, 0x02]);
    d.extend([0x05, 0x09, 0x09, 0x01, 0x25, 0x01, 0x75, 0x01, 0x81, 0x02, 0x95, 0x07, 0x81, 0x03]);
    d.push(0xC0);
    d
}

/// A finger in a report: down or not, its id, where it is.
pub fn at(tip: bool, id: u8, x: u16, y: u16) -> [u8; 6] {
    let (x, y) = (x.to_le_bytes(), y.to_le_bytes());
    [tip as u8 | 0x02, id, x[0], x[1], y[0], y[1]]
}

pub fn report(fingers: &[[u8; 6]], count: u8, button: bool) -> Vec<u8> {
    let mut r = vec![RID];
    fingers.iter().for_each(|f| r.extend(f));
    r.extend([count, button as u8]);
    r
}
