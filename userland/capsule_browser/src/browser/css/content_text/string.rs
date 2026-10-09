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

/* The quoted string at the start of `s` with its escapes decoded, and
 * the bytes it spans through the closing quote (or the end). A hex
 * escape ("\e90d", as icon fonts write their glyphs) stands for its code
 * point and eats one following space; any other escaped character
 * stands for itself; an escaped newline continues the string. */
pub(super) fn string_token(s: &str) -> (String, usize) {
    let quote = s.chars().next().unwrap_or('"');
    let mut out = String::new();
    let mut chars = s.char_indices().skip(1).peekable();
    while let Some((i, ch)) = chars.next() {
        if ch == quote {
            return (out, i + 1);
        }
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        let mut hex = 0u32;
        let mut digits = 0;
        while digits < 6 {
            match chars.peek().and_then(|&(_, c)| c.to_digit(16)) {
                Some(d) => {
                    hex = hex * 16 + d;
                    digits += 1;
                    chars.next();
                }
                None => break,
            }
        }
        if digits == 0 {
            match chars.next() {
                Some((_, '\n')) | None => {}
                Some((_, c)) => out.push(c),
            }
            continue;
        }
        if chars.peek().is_some_and(|&(_, c)| c == ' ') {
            chars.next();
        }
        out.push(char::from_u32(hex).unwrap_or('\u{fffd}'));
    }
    (out, s.len())
}
