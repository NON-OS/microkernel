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
use super::ident::name_char;
use super::nth_num::{clamp, number, offset, signed};

/* The An+B microsyntax (CSS Syntax 6): odd, even, an integer, or A n with
 * an optional +B or -B. A sign must touch the number or n it signs, and no
 * space may split a number from its n; around the sign of B spaces are
 * free. Literals too large for i32 saturate instead of wrapping. */
pub(super) fn anb(c: &mut Cur) -> Option<(i32, i32)> {
    for (word, v) in [("odd", (2, 1)), ("even", (2, 0))] {
        let end = c.i + word.len();
        let hit = c.s.get(c.i..end).is_some_and(|w| w.eq_ignore_ascii_case(word));
        if hit && !c.at(word.len()).is_some_and(name_char) {
            c.i = end;
            return Some(v);
        }
    }
    let neg = c.peek() == Some(b'-');
    if neg || c.peek() == Some(b'+') {
        c.i += 1;
    }
    let digits = number(c);
    let (a, b) = if matches!(c.peek(), Some(b'n' | b'N')) {
        c.i += 1;
        (signed(neg, digits.unwrap_or(1)), offset(c)?)
    } else {
        (0, signed(neg, digits?))
    };
    if c.peek().is_some_and(name_char) {
        return None;
    }
    Some((clamp(a), clamp(b)))
}
