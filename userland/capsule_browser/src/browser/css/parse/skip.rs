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

use super::cursor::Cur;

/* Step past the rest of one entry of a selector list: to the next comma at
 * this level, or (unless `top`) to the parenthesis closing the list, or to
 * the end. Strings, escapes, comments and inner brackets are crossed whole,
 * so a comma inside :is(a, b) or [x=","] never splits. One pass, linear in
 * the text however deep the brackets go. */
pub(super) fn skip_arg(c: &mut Cur, top: bool) {
    let mut depth = 0u32;
    while let Some(b) = c.peek() {
        match b {
            b'(' | b'[' => depth += 1,
            b')' | b']' if depth > 0 => depth -= 1,
            b')' if !top => return,
            b',' if depth == 0 => return,
            b'"' | b'\'' => {
                skip_string(c, b);
                continue;
            }
            b'\\' => c.i += 1,
            b'/' if c.at(1) == Some(b'*') => {
                c.comments();
                continue;
            }
            _ => {}
        }
        c.i += 1;
    }
    /* A byte step can stop inside a multi-byte character only at the end,
     * where the whole text has been consumed anyway. */
    c.i = c.s.len().min(c.i);
}

/* From an opening quote to its closing quote, a raw newline (which ends a
 * bad string) or the end. */
fn skip_string(c: &mut Cur, q: u8) {
    c.i += 1;
    while let Some(b) = c.peek() {
        c.i += 1;
        match b {
            b'\\' => c.i += 1,
            b'\n' | b'\r' | 0x0c => return,
            _ if b == q => return,
            _ => {}
        }
    }
    c.i = c.s.len().min(c.i);
}
