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

use alloc::string::String;

use super::index::{at, push, JIS0208};

/* The Shift_JIS decoder (windows-31J): ASCII and 0x80 as themselves,
half-width katakana at 0xA1..=0xDF, else a lead and a trail through
index jis0208, whose pointers 8836..=10715 are the user-defined area
Windows maps to U+E000 onward. */
pub fn decode(b: &[u8], out: &mut String) {
    let mut i = 0;
    while let Some(&lead) = b.get(i) {
        i += 1;
        match lead {
            0x00..=0x80 => out.push(char::from(lead)),
            0xA1..=0xDF => push(out, Some(0xFF61 - 0xA1 + u32::from(lead))),
            0x81..=0x9F | 0xE0..=0xFC => {
                let Some(&byte) = b.get(i) else { return out.push('\u{FFFD}') };
                let offset = if byte < 0x7F { 0x40 } else { 0x41 };
                let lead_offset = if lead < 0xA0 { 0x81 } else { 0xC1 };
                let pointer = matches!(byte, 0x40..=0x7E | 0x80..=0xFC)
                    .then(|| (usize::from(lead) - lead_offset) * 188 + usize::from(byte) - offset);
                let c = match pointer {
                    Some(p @ 8836..=10715) => Some(0xE000 - 8836 + p as u32),
                    Some(p) => at(JIS0208, p),
                    None => None,
                };
                push(out, c);
                /* An ASCII byte after a lead that forms nothing is read again. */
                i += usize::from(c.is_some() || byte >= 0x80);
            }
            _ => out.push('\u{FFFD}'),
        }
    }
}
