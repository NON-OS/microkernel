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

//! One IVHD block's device entries (AMD IOMMU spec 48882, 5.2.2.2). Entry
//! size follows the type: 4 bytes below 0x40, 8 below 0x80, 16 below 0xC0,
//! and an ACPI HID entry (0xF0) is 22 bytes plus its UID.

use super::ivhd_kind::spans_of;
use super::ivhd_scope::Span;

fn word(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([bytes[at], bytes[at + 1]])
}

fn size(bytes: &[u8], at: usize) -> Option<usize> {
    match bytes[at] {
        0x00..=0x3F => Some(4),
        0x40..=0x7F => Some(8),
        0x80..=0xBF => Some(16),
        0xF0 if at + 22 <= bytes.len() => Some(22 + bytes[at + 21] as usize),
        _ => None,
    }
}

/// Append the spans `bytes` describes to `out[n..]`, returning the new count.
pub(super) fn entries(bytes: &[u8], out: &mut [Span], mut n: usize) -> usize {
    let (mut at, mut start) = (0, None);
    while at + 4 <= bytes.len() {
        let Some(len) = size(bytes, at).filter(|len| at + len <= bytes.len()) else {
            break;
        };
        let alias = if len >= 8 { word(bytes, at + 5) } else { 0 };
        for span in
            spans_of(bytes[at], word(bytes, at + 1), alias, &mut start).into_iter().flatten()
        {
            if n == out.len() {
                return n;
            }
            out[n] = span;
            n += 1;
        }
        at += len;
    }
    n
}
