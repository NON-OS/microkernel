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

use super::index::{at, push, JIS0208, JIS0212};

/* The EUC-JP decoder: ASCII; 0x8E and a half-width katakana byte; 0x8F
and two bytes through index jis0212; two bytes 0xA1..=0xFE through
index jis0208. */
pub fn decode(b: &[u8], out: &mut String) {
    let mut i = 0;
    while let Some(&first) = b.get(i) {
        i += 1;
        if first < 0x80 {
            out.push(char::from(first));
            continue;
        }
        if !(first == 0x8E || first == 0x8F || (0xA1..=0xFE).contains(&first)) {
            out.push('\u{FFFD}');
            continue;
        }
        let Some(&second) = b.get(i) else { return out.push('\u{FFFD}') };
        if first == 0x8E && (0xA1..=0xDF).contains(&second) {
            push(out, Some(0xFF61 - 0xA1 + u32::from(second)));
            i += 1;
            continue;
        }
        let (lead, byte, table) = if first == 0x8F && (0xA1..=0xFE).contains(&second) {
            i += 1;
            let Some(&third) = b.get(i) else { return out.push('\u{FFFD}') };
            (second, third, JIS0212)
        } else {
            (first, second, JIS0208)
        };
        let c = ((0xA1..=0xFE).contains(&lead) && (0xA1..=0xFE).contains(&byte))
            .then(|| (usize::from(lead) - 0xA1) * 94 + usize::from(byte) - 0xA1)
            .and_then(|p| at(table, p));
        push(out, c);
        /* An ASCII byte after a lead that forms nothing is read again. */
        i += usize::from(c.is_some() || byte >= 0x80);
    }
}
