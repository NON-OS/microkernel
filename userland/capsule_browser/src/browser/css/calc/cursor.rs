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

pub(in crate::browser::css) struct P<'a> {
    pub(in crate::browser::css) s: &'a [u8],
    pub(in crate::browser::css) i: usize,
    pub(in crate::browser::css) em: f32,
    /* Current parenthesis/function nesting depth. Bounds the recursive
     * descent so a crafted `calc(((((...)))))` cannot overflow the stack. */
    pub(in crate::browser::css) d: u32,
}

/* Deepest nesting accepted; well past any real stylesheet. */
pub(in crate::browser::css) const MAX_CALC_DEPTH: u32 = 32;

impl P<'_> {
    pub(in crate::browser::css) fn skip_ws(&mut self) {
        while self.s.get(self.i).is_some_and(|b| b.is_ascii_whitespace()) {
            self.i += 1;
        }
    }
}

/// One min()/max()/clamp() argument part rounded half away from zero to
/// whole px (or per-mille), None past the 16-bit range an argument keeps.
pub(super) fn lin(v: f32) -> Option<i16> {
    let half = if v < 0.0 { -0.5 } else { 0.5 };
    (v.is_finite() && v.abs() <= i16::MAX as f32).then(|| (v + half) as i16)
}
