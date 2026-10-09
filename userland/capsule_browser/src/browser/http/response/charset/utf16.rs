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

use super::index::push;

/* The shared UTF-16 decoder: code units from byte pairs, surrogate pairs
joined. A leading surrogate not followed by a trailing one is an error
and the unit after it is read again; a lone trailing surrogate is an
error; an odd byte or a leading surrogate at the end is one error. */
pub fn decode(b: &[u8], be: bool, out: &mut String) {
    let mut lead: Option<u32> = None;
    let mut i = 0;
    while i + 1 < b.len() {
        let pair = [b[i], b[i + 1]];
        let unit = u32::from(if be { u16::from_be_bytes(pair) } else { u16::from_le_bytes(pair) });
        i += 2;
        if let Some(l) = lead.take() {
            if (0xDC00..=0xDFFF).contains(&unit) {
                push(out, Some(0x10000 + ((l - 0xD800) << 10) + (unit - 0xDC00)));
            } else {
                out.push('\u{FFFD}');
                i -= 2;
            }
        } else if (0xD800..=0xDBFF).contains(&unit) {
            lead = Some(unit);
        } else if (0xDC00..=0xDFFF).contains(&unit) {
            out.push('\u{FFFD}');
        } else {
            push(out, Some(unit));
        }
    }
    if lead.is_some() || i < b.len() {
        out.push('\u{FFFD}');
    }
}
