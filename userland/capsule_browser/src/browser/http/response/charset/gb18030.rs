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

use super::gb18030_ranges::ranges_code_point;
use super::index::{at, push};

static INDEX: &[u8] = include_bytes!("index/gb18030.idx");

/* The gb18030 decoder (also GBK's): ASCII, 0x80 as the euro sign, two
bytes through the index, four bytes through the ranges. Where a
sequence breaks, the bytes after its lead are read again unless the
standard consumes them; a sequence cut by the end is one error. */
pub fn decode(b: &[u8], out: &mut String) {
    let mut i = 0;
    while let Some(&first) = b.get(i) {
        i += 1;
        match first {
            0x00..=0x7F => out.push(char::from(first)),
            0x80 => out.push('\u{20AC}'),
            0xFF => out.push('\u{FFFD}'),
            _ => {
                let Some(&second) = b.get(i) else { return out.push('\u{FFFD}') };
                if !(0x30..=0x39).contains(&second) {
                    let offset = if second < 0x7F { 0x40 } else { 0x41 };
                    let pointer = (matches!(second, 0x40..=0x7E | 0x80..=0xFE))
                        .then(|| (usize::from(first) - 0x81) * 190 + usize::from(second) - offset);
                    let c = pointer.and_then(|p| at(INDEX, p));
                    push(out, c);
                    /* An ASCII byte after a bad lead is read again. */
                    i += usize::from(c.is_some() || second >= 0x80);
                    continue;
                }
                let (Some(&third), Some(&fourth)) = (b.get(i + 1), b.get(i + 2)) else {
                    /* Within the bytes there are, a bad third byte is found first. */
                    if b.get(i + 1).is_some_and(|t| !(0x81..=0xFE).contains(t)) {
                        out.push('\u{FFFD}');
                        continue;
                    }
                    return out.push('\u{FFFD}');
                };
                if !(0x81..=0xFE).contains(&third) || !(0x30..=0x39).contains(&fourth) {
                    out.push('\u{FFFD}');
                    continue;
                }
                let pointer = (u32::from(first) - 0x81) * 12600
                    + (u32::from(second) - 0x30) * 1260
                    + (u32::from(third) - 0x81) * 10
                    + u32::from(fourth)
                    - 0x30;
                push(out, ranges_code_point(pointer));
                i += 3;
            }
        }
    }
}
