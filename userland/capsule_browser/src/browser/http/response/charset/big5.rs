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

use super::index::{at, push};

static INDEX: &[u8] = include_bytes!("index/big5.idx");
/* One bit per pointer: its code point is in plane 2, U+20000 plus the
16 bits the index holds. */
static ASTRAL: &[u8] = include_bytes!("index/big5_astral.idx");

fn code_point(pointer: usize) -> Option<u32> {
    let c = at(INDEX, pointer)?;
    let astral = ASTRAL.get(pointer / 8).is_some_and(|&m| (m >> (pointer % 8)) & 1 != 0);
    Some(if astral { 0x20000 | c } else { c })
}

/* The Big5 decoder (Big5 with HKSCS): ASCII, or a lead 0x81..=0xFE and
a trail byte. Four pointers decode to a letter and a combining mark. */
pub fn decode(b: &[u8], out: &mut String) {
    let mut i = 0;
    while let Some(&lead) = b.get(i) {
        i += 1;
        if lead < 0x80 {
            out.push(char::from(lead));
            continue;
        }
        if !(0x81..=0xFE).contains(&lead) {
            out.push('\u{FFFD}');
            continue;
        }
        let Some(&byte) = b.get(i) else { return out.push('\u{FFFD}') };
        let offset = if byte < 0x7F { 0x40 } else { 0x62 };
        let pointer = matches!(byte, 0x40..=0x7E | 0xA1..=0xFE)
            .then(|| (usize::from(lead) - 0x81) * 157 + usize::from(byte) - offset);
        let pair = match pointer {
            Some(1133) => Some("\u{CA}\u{304}"),
            Some(1135) => Some("\u{CA}\u{30C}"),
            Some(1164) => Some("\u{EA}\u{304}"),
            Some(1166) => Some("\u{EA}\u{30C}"),
            _ => None,
        };
        if let Some(pair) = pair {
            out.push_str(pair);
            i += 1;
            continue;
        }
        let c = pointer.and_then(code_point);
        push(out, c);
        /* An ASCII byte after a lead that forms nothing is read again. */
        i += usize::from(c.is_some() || byte >= 0x80);
    }
}
