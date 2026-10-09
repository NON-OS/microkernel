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

use super::cursor::P;
use super::expr::V;
use super::unit::unit_px;

/// A number with an optional unit or percent sign at the cursor.
pub(in crate::browser::css) fn number(p: &mut P) -> Option<V> {
    let start = p.i;
    if p.s.get(p.i).is_some_and(|&b| b == b'+' || b == b'-') {
        p.i += 1;
    }
    let digits = p.i;
    let mut dot = false;
    while p.s.get(p.i).is_some_and(|&b| b.is_ascii_digit() || (b == b'.' && !dot)) {
        dot |= p.s[p.i] == b'.';
        p.i += 1;
    }
    if p.i == digits {
        return None;
    }
    /* An exponent needs a digit after the e (and its sign), which keeps the
     * em and ex units apart from 1e3. */
    let at = |k: usize| p.s.get(p.i + k).copied().unwrap_or(0);
    if at(0) | 0x20 == b'e' {
        let skip = if at(1) == b'+' || at(1) == b'-' { 2 } else { 1 };
        if at(skip).is_ascii_digit() {
            p.i += skip;
            while p.s.get(p.i).is_some_and(|b| b.is_ascii_digit()) {
                p.i += 1;
            }
        }
    }
    let n: f32 = core::str::from_utf8(&p.s[start..p.i]).ok()?.parse().ok()?;
    let ustart = p.i;
    while p.s.get(p.i).is_some_and(|b| b.is_ascii_alphabetic() || *b == b'%') {
        p.i += 1;
    }
    match &p.s[ustart..p.i].to_ascii_lowercase()[..] {
        b"" => Some(V::Num(n)),
        b"%" => Some(V::Len { px: 0.0, pml: n * 10.0 }),
        unit => Some(V::Len { px: n * unit_px(unit, p.em)?, pml: 0.0 }),
    }
}
