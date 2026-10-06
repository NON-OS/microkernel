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

use super::cursor::Cur;
use super::escape::escape;

/* Consume the quoted string whose opening quote is at c.i (CSS Syntax
 * 4.3.5). A raw newline makes a bad string, which is None; the end of input
 * closes the string; an escaped newline continues it. */
pub(super) fn string(c: &mut Cur) -> Option<String> {
    let q = c.peek()?;
    c.i += 1;
    let mut out = String::new();
    loop {
        let b = c.s.as_bytes();
        match b.get(c.i).copied() {
            None => return Some(out),
            Some(x) if x == q => {
                c.i += 1;
                return Some(out);
            }
            Some(b'\n' | b'\r' | 0x0c) => return None,
            Some(b'\\') => match b.get(c.i + 1) {
                None => c.i += 1,
                Some(b'\r') => {
                    c.i += 2;
                    c.eat(b'\n');
                }
                Some(b'\n' | 0x0c) => c.i += 2,
                Some(_) => out.push(escape(c)),
            },
            Some(0) => {
                out.push('\u{FFFD}');
                c.i += 1;
            }
            Some(_) => {
                let ch = c.s[c.i..].chars().next()?;
                out.push(ch);
                c.i += ch.len_utf8();
            }
        }
    }
}
