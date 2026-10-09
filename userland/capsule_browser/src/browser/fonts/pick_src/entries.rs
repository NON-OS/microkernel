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

use alloc::vec::Vec;

/// The pieces of `src` between `sep` bytes that sit outside parentheses
/// and quotes, so a data: URI in a url() stays whole.
pub(in crate::browser::fonts) fn split_top(src: &str, sep: u8) -> Vec<&str> {
    let (mut out, mut depth, mut quote, mut start) = (Vec::new(), 0i32, 0u8, 0);
    for (i, b) in src.bytes().enumerate() {
        match b {
            _ if quote != 0 => quote = if b == quote { 0 } else { quote },
            b'"' | b'\'' => quote = b,
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ if b == sep && depth <= 0 => {
                out.push(&src[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(&src[start..]);
    out
}
