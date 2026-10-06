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
use super::escape::{escape, valid_escape};

/* A letter, underscore, non-ASCII byte or NUL (read as U+FFFD). */
fn name_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_' || b >= 0x80 || b == 0
}

pub(super) fn name_char(b: u8) -> bool {
    name_start(b) || b.is_ascii_digit() || b == b'-'
}

/* Whether an identifier starts at c.i (CSS Syntax 4.3.9): a name-start code
 * point or escape, or a hyphen followed by one of those or by a hyphen. */
pub(super) fn starts_ident(c: &Cur) -> bool {
    let b = c.s.as_bytes();
    match c.peek() {
        Some(b'-') => match c.at(1) {
            Some(b'-') => true,
            Some(b'\\') => valid_escape(b, c.i + 1),
            Some(x) => name_start(x),
            None => false,
        },
        Some(b'\\') => valid_escape(b, c.i),
        Some(x) => name_start(x),
        None => false,
    }
}

/* Consume an identifier and return it with escapes decoded, or None when
 * none starts here. */
pub(super) fn ident(c: &mut Cur) -> Option<String> {
    if !starts_ident(c) {
        return None;
    }
    let mut out = String::new();
    loop {
        let b = c.s.as_bytes();
        match b.get(c.i).copied() {
            Some(b'\\') if valid_escape(b, c.i) => out.push(escape(c)),
            Some(0) => {
                out.push('\u{FFFD}');
                c.i += 1;
            }
            Some(x) if x < 0x80 && name_char(x) => {
                out.push(x as char);
                c.i += 1;
            }
            Some(x) if x >= 0x80 => {
                let ch = c.s[c.i..].chars().next()?;
                out.push(ch);
                c.i += ch.len_utf8();
            }
            _ => return Some(out),
        }
    }
}
