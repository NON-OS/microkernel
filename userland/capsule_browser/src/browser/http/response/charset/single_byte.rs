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

/* The 28 single-byte indexes, 128 code points each, in labels.txt order. */
static TABLES: &[u8] = include_bytes!("index/single_byte.idx");

/* The single-byte decoder: ASCII as is, 0x80..=0xFF through the table. */
pub fn decode(table: usize, b: &[u8], out: &mut String) {
    let t = TABLES.get(table * 256..table * 256 + 256).unwrap_or(&[]);
    for &c in b {
        if c < 0x80 {
            out.push(char::from(c));
        } else {
            push(out, at(t, usize::from(c - 0x80)));
        }
    }
}

/* x-user-defined: ASCII as is, 0x80..=0xFF to U+F780..=U+F7FF. */
pub fn user_defined(b: &[u8], out: &mut String) {
    for &c in b {
        let cp = if c < 0x80 { u32::from(c) } else { 0xF780 + u32::from(c) - 0x80 };
        push(out, Some(cp));
    }
}
