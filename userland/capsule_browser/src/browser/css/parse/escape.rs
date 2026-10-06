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

use crate::browser::css::selector::is_space;

use super::cursor::Cur;

/* A backslash at `i` starts an escape unless a newline follows it; the end
 * of input may follow it (CSS Syntax 4.3.8). */
pub(super) fn valid_escape(s: &[u8], i: usize) -> bool {
    s.get(i) == Some(&b'\\') && !matches!(s.get(i + 1), Some(b'\n' | b'\r' | 0x0c))
}

/* Decode the escape whose backslash is at c.i (CSS Syntax 4.3.7): one to
 * six hex digits and one optional whitespace, or else the next code point
 * itself. Zero, surrogates, values past U+10FFFF and a backslash at the end
 * all decode to U+FFFD. */
pub(super) fn escape(c: &mut Cur) -> char {
    c.i += 1;
    let start = c.i;
    let mut v: u32 = 0;
    while c.i - start < 6 {
        let Some(d) = c.peek().and_then(|b| (b as char).to_digit(16)) else {
            break;
        };
        v = v * 16 + d;
        c.i += 1;
    }
    if c.i > start {
        if c.eat(b'\r') {
            c.eat(b'\n');
        } else if c.peek().is_some_and(is_space) {
            c.i += 1;
        }
        return match char::from_u32(v) {
            Some(ch) if v != 0 => ch,
            _ => '\u{FFFD}',
        };
    }
    match c.s[c.i..].chars().next() {
        Some('\0') => {
            c.i += 1;
            '\u{FFFD}'
        }
        Some(ch) => {
            c.i += ch.len_utf8();
            ch
        }
        None => '\u{FFFD}',
    }
}
