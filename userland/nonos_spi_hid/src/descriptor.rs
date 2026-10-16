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
//! The device descriptor an SPI-HID device answers with: its descriptor and
//! report descriptor lengths, the most it sends and takes, and its ids.

use super::facts::DEVICE_DESCRIPTOR_LEN;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Descriptor {
    pub version: u16,
    pub report_desc_len: u16,
    pub max_input: u16,
    pub max_output: u16,
    pub max_fragment: u16,
    pub vendor: u16,
    pub product: u16,
}

/// The descriptor in `d`, which must say its own length is the one known.
pub fn descriptor(d: &[u8]) -> Option<Descriptor> {
    let d = d.get(..DEVICE_DESCRIPTOR_LEN)?;
    let w = |i: usize| u16::from_le_bytes([d[2 * i], d[2 * i + 1]]);
    if w(0) as usize != DEVICE_DESCRIPTOR_LEN {
        return None;
    }
    Some(Descriptor {
        version: w(1),
        report_desc_len: w(2),
        max_input: w(3),
        max_output: w(4),
        max_fragment: w(5),
        vendor: w(6),
        product: w(7),
    })
}
