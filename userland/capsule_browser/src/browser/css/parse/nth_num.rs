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

/* The +B or -B after n, or 0 when neither follows. */
pub(super) fn offset(c: &mut Cur) -> Option<i64> {
    let save = c.i;
    c.trivia();
    let neg = match c.peek() {
        Some(b'+') => false,
        Some(b'-') => true,
        _ => {
            c.i = save;
            return Some(0);
        }
    };
    c.i += 1;
    c.trivia();
    Some(signed(neg, number(c)?))
}

pub(super) fn number(c: &mut Cur) -> Option<i64> {
    let start = c.i;
    let mut v: i64 = 0;
    while let Some(d) = c.peek().filter(u8::is_ascii_digit) {
        v = v.saturating_mul(10).saturating_add((d - b'0') as i64);
        c.i += 1;
    }
    (c.i > start).then_some(v)
}

pub(super) fn signed(neg: bool, v: i64) -> i64 {
    if neg {
        -v
    } else {
        v
    }
}

pub(super) fn clamp(v: i64) -> i32 {
    v.clamp(i32::MIN as i64, i32::MAX as i64) as i32
}
