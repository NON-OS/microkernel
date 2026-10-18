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

//! The bus numbers a PCI root bridge decodes, from its _CRS: the Word,
//! DWord or QWord Address Space descriptor of the bus number type (2)
//! (ACPI 6.5, 6.4.3.5), its minimum and length. The first such descriptor
//! counts; None when there is none or it names no bus, or more than 256.

use super::template::items;

fn le(b: &[u8]) -> u64 {
    b.iter().rev().fold(0, |a, x| a << 8 | u64::from(*x))
}

/// The first and last bus.
pub fn bus_range(crs: &[u8]) -> Option<(u8, u8)> {
    for i in items(crs).iter().filter(|i| i.large) {
        let (w, at, least) = match i.kind {
            0x08 => (2, 5, 13),
            0x07 => (4, 7, 23),
            0x0A => (8, 11, 43),
            _ => continue,
        };
        let b = i.body;
        if b.len() < least || b[0] != 2 {
            continue;
        }
        let (min, len) = (le(&b[at..at + w]), le(&b[at + 3 * w..at + 4 * w]));
        let last = min.checked_add(len)?.checked_sub(1)?;
        return Some((u8::try_from(min).ok()?, u8::try_from(last).ok()?));
    }
    None
}
