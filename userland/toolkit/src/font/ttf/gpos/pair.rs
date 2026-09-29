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

use super::coverage::{class_of, coverage};
use super::read::{last_at_or_below, u16_at};

/// The x advance adjustment, in font units, a PairPos subtable at `sub`
/// makes to glyph `a` followed by `b`; None when it does not cover the
/// pair. Format 1 lists second glyphs per first glyph, format 2 a value per
/// pair of glyph classes. Only the first glyph's XAdvance is read, which
/// is where kerning lives.
pub(super) fn pair_x(d: &[u8], sub: usize, a: u16, b: u16) -> Option<i16> {
    let idx = coverage(d, sub + u16_at(d, sub + 2)? as usize, a)? as usize;
    let (vf1, vf2) = (u16_at(d, sub + 4)?, u16_at(d, sub + 6)?);
    let (size1, size2) =
        (2 * (vf1 & 0xff).count_ones() as usize, 2 * (vf2 & 0xff).count_ones() as usize);
    let x_adv = |rec: usize| -> Option<i16> {
        (vf1 & 4 != 0)
            .then(|| u16_at(d, rec + 2 * (vf1 & 3).count_ones() as usize).map(|v| v as i16))?
    };
    match u16_at(d, sub)? {
        1 => {
            let set = sub + u16_at(d, sub + 10 + 2 * idx)? as usize;
            let (n, step) = (u16_at(d, set)? as usize, 2 + size1 + size2);
            let i = last_at_or_below(n, b, |i| u16_at(d, set + 2 + step * i))?;
            let rec = set + 2 + step * i;
            (u16_at(d, rec)? == b).then(|| x_adv(rec + 2))?
        }
        2 => {
            let (c1, c2) = (
                class_of(d, sub + u16_at(d, sub + 8)? as usize, a) as usize,
                class_of(d, sub + u16_at(d, sub + 10)? as usize, b) as usize,
            );
            let (n1, n2) = (u16_at(d, sub + 12)? as usize, u16_at(d, sub + 14)? as usize);
            (c1 < n1 && c2 < n2).then(|| x_adv(sub + 16 + (c1 * n2 + c2) * (size1 + size2)))?
        }
        _ => None,
    }
}
