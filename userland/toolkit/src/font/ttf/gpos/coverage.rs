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

use super::read::{last_at_or_below, u16_at};

/* The coverage index of glyph `g` in the Coverage table at `at`: format 1
 * lists the glyphs, format 2 ranges with the index of each first glyph.
 * Both are sorted by glyph. */
pub(super) fn coverage(d: &[u8], at: usize, g: u16) -> Option<u16> {
    let n = u16_at(d, at + 2)? as usize;
    match u16_at(d, at)? {
        1 => {
            let i = last_at_or_below(n, g, |i| u16_at(d, at + 4 + 2 * i))?;
            (u16_at(d, at + 4 + 2 * i)? == g).then_some(i as u16)
        }
        2 => {
            let r = |i: usize| at + 4 + 6 * i;
            let i = last_at_or_below(n, g, |i| u16_at(d, r(i)))?;
            let (first, last) = (u16_at(d, r(i))?, u16_at(d, r(i) + 2)?);
            (g <= last).then(|| u16_at(d, r(i) + 4).map(|s| s.wrapping_add(g - first)))?
        }
        _ => None,
    }
}

/// The class of glyph `g` in the ClassDef table at `at`: format 1 gives a
/// class per glyph from a start glyph, format 2 a class per sorted range.
/// A glyph not listed is class 0.
pub(super) fn class_of(d: &[u8], at: usize, g: u16) -> u16 {
    let class = || -> Option<u16> {
        match u16_at(d, at)? {
            1 => {
                let (start, n) = (u16_at(d, at + 2)?, u16_at(d, at + 4)?);
                let i = g.checked_sub(start).filter(|&i| i < n)?;
                u16_at(d, at + 6 + 2 * i as usize)
            }
            2 => {
                let (n, r) = (u16_at(d, at + 2)? as usize, |i: usize| at + 4 + 6 * i);
                let i = last_at_or_below(n, g, |i| u16_at(d, r(i)))?;
                (g <= u16_at(d, r(i) + 2)?).then(|| u16_at(d, r(i) + 4))?
            }
            _ => None,
        }
    };
    class().unwrap_or(0)
}
