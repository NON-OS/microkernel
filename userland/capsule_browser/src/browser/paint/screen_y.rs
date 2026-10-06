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

/* Where a fragment's top lands in the viewport, 0 being the first page row,
 * with the page scrolled by `scroll`. A fixed fragment ignores the scroll and
 * pins to the viewport; a sticky subtree anchored at flow y `anchor` with
 * offset `top` shifts down once the scroll passes its threshold; everything
 * else scrolls with the page. Paint and the hit tests both use this, so a
 * click finds what was drawn where it was drawn. */
pub fn frag_screen_y(y: i32, fixed: bool, sticky: Option<(i32, i32)>, scroll: i32) -> i32 {
    if fixed {
        return y;
    }
    let sy = y.saturating_sub(scroll);
    match sticky {
        Some((anchor, top)) => {
            sy.saturating_add(scroll.saturating_sub(anchor).saturating_add(top).max(0))
        }
        None => sy,
    }
}
