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

/* The index tables of the Encoding Standard, as scripts/
gen_encoding_index.py writes them from the published index files:
little-endian u16 code points by pointer, 0 where there is none. */
pub static JIS0208: &[u8] = include_bytes!("index/jis0208.idx");
pub static JIS0212: &[u8] = include_bytes!("index/jis0212.idx");

/* The code point at `pointer` of `table`, or None. */
pub fn at(table: &[u8], pointer: usize) -> Option<u32> {
    let i = pointer.checked_mul(2)?;
    let b = table.get(i..i.checked_add(2)?)?;
    let c = u16::from_le_bytes([b[0], b[1]]);
    (c != 0).then_some(u32::from(c))
}

/* Appends code point `c`, or U+FFFD for an error. */
pub fn push(out: &mut String, c: Option<u32>) {
    out.push(c.and_then(char::from_u32).unwrap_or('\u{FFFD}'));
}
