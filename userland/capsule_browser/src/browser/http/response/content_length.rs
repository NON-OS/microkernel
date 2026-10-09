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

use super::bytes::list;

/* Content-Length as the fields so far give it. */
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Length {
    Absent,
    Is(u64),
    /* Not 1*DIGIT, too large, or two different values: RFC 9112 6.3
    makes the framing an unrecoverable error. */
    Invalid,
}

/* Folds one Content-Length field value in. A comma list of the same
number, or the same number in several fields, is that number (RFC
9110 8.6); a sign, a space inside or any other value is invalid. */
pub fn content_length(cur: &mut Length, value: &[u8]) {
    let mut any = false;
    for item in list(value) {
        any = true;
        let n = item.iter().try_fold(0u64, |v, &c| {
            let d = c.is_ascii_digit().then(|| u64::from(c - b'0'))?;
            v.checked_mul(10)?.checked_add(d)
        });
        *cur = match (*cur, n) {
            (Length::Absent, Some(n)) => Length::Is(n),
            (Length::Is(old), Some(n)) if old == n => Length::Is(n),
            _ => Length::Invalid,
        };
    }
    if !any {
        *cur = Length::Invalid;
    }
}
