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

/* The sheet with its comments cut out. A comment opener inside a quoted
 * string is text (content: "/" "*" written together), so strings are
 * copied through whole; a string also ends at a newline, as a bad string
 * does in CSS. An unclosed comment runs to the end of the sheet. */
pub fn strip_comments(css: &str) -> String {
    let b = css.as_bytes();
    let mut out = String::with_capacity(css.len());
    let (mut quote, mut start, mut i) = (0u8, 0usize, 0usize);
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
        } else if c == b'"' || c == b'\'' {
            quote = c;
        } else if c == b'/' && b.get(i + 1) == Some(&b'*') {
            out.push_str(&css[start..i]);
            i = match css[i + 2..].find("*/") {
                Some(end) => i + 2 + end + 2,
                None => b.len(),
            };
            start = i;
            continue;
        }
        i += 1;
    }
    out.push_str(css.get(start..).unwrap_or(""));
    out
}
