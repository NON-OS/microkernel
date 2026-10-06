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

/* Byte index of the brace closing an already-opened block, honoring nested
 * blocks and skipping braces inside quoted strings (content: "}"). A string
 * also ends at a newline, as a bad string does. Unbalanced input runs to
 * the end. */
pub(super) fn matching_brace(s: &str) -> usize {
    let b = s.as_bytes();
    let (mut depth, mut quote, mut i) = (1u32, 0u8, 0usize);
    while i < b.len() {
        let c = b[i];
        if c == b'\\' {
            i += 2;
            continue;
        }
        if quote != 0 {
            if c == quote || c == b'\n' {
                quote = 0;
            }
        } else {
            match c {
                b'"' | b'\'' => quote = c,
                b'{' => depth = depth.saturating_add(1),
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        return i;
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    s.len()
}
