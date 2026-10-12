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
//! What a HID interface is for, from its report descriptor (HID 1.11,
//! 6.2.2, as recalled, unverified): the usage page and usage in force at
//! its first application collection. Only a Generic Desktop pointer or
//! mouse is fed to the tablet decoder; a security key (page F1D0h), a
//! gamepad or a vendor channel is left to its own driver.

const USAGE_PAGE: u8 = 0x04;
const USAGE: u8 = 0x08;
const COLLECTION: u8 = 0xA0;
const APPLICATION: u32 = 0x01;
const LONG_ITEM: u8 = 0xFE;
const GENERIC_DESKTOP: u16 = 0x01;
const POINTER: u16 = 0x01;
const MOUSE: u16 = 0x02;

/// The (usage page, usage) of the first application collection.
pub fn application(report: &[u8]) -> Option<(u16, u16)> {
    let (mut page, mut usage, mut i) = (0u16, None, 0usize);
    while let Some(&prefix) = report.get(i) {
        if prefix == LONG_ITEM {
            i += 3 + *report.get(i + 1)? as usize;
            continue;
        }
        let size = [0, 1, 2, 4][(prefix & 0x03) as usize];
        let data = report.get(i + 1..i + 1 + size)?;
        let value = data.iter().rev().fold(0u32, |v, &b| v << 8 | b as u32);
        match prefix & 0xFC {
            USAGE_PAGE => page = value as u16,
            // A four-byte usage carries its own page in the upper half.
            USAGE if size == 4 => usage = Some(((value >> 16) as u16, value as u16)),
            USAGE => usage = Some((page, value as u16)),
            COLLECTION if value == APPLICATION => return usage,
            _ => {}
        }
        i += 1 + size;
    }
    None
}

/// Whether the tablet decoder may read this interface's reports.
pub fn is_pointer(report: &[u8]) -> bool {
    matches!(application(report), Some((GENERIC_DESKTOP, POINTER | MOUSE)))
}
