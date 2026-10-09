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

use super::containing::Containing;

/* position:relative shift after normal flow: left/top win over right/bottom.
 * left and right percentages take the containing block's width, top and
 * bottom its height; while that height is unknown they shift by nothing. */
pub(crate) fn rel_offset(s: &Computed, cb: Containing) -> (i32, i32) {
    let h = cb.h.unwrap_or(0);
    let dx = match s.left.resolve(cb.w) {
        Some(l) => l,
        None => -s.right.resolve(cb.w).unwrap_or(0),
    };
    let dy = match s.top.resolve(h) {
        Some(t) => t,
        None => -s.bottom.resolve(h).unwrap_or(0),
    };
    (dx, dy)
}
