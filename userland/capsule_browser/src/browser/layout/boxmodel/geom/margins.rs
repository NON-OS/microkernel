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

use crate::browser::css::Computed;

/// The margins [top, right, bottom, left] of a box whose containing block
/// is `base` px wide: each side's px part plus its percentage part, which
/// CSS takes from the containing block's width on every side.
pub(crate) fn margins(s: &Computed, base: i32) -> [i32; 4] {
    let px = [s.margin_top, s.margin_right, s.margin_bottom, s.margin_left];
    core::array::from_fn(|i| {
        let pct = (base as i64 * s.margin_pml[i] as i64 / 1000).clamp(-1 << 30, 1 << 30);
        px[i].saturating_add(pct as i32)
    })
}

/* Two adjoining vertical margins collapse to the largest positive one plus
 * the most negative one. */
pub(crate) fn collapse(a: i32, b: i32) -> i32 {
    a.max(b).max(0) + a.min(b).min(0)
}
