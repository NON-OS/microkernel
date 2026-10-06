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

use crate::browser::css::{Computed, Overflow, Size};

/* aspect-ratio: an auto height follows the border-box width `w` (content
 * width `cw`). The ratio applies to the box box-sizing names, so a
 * content-box ratio adds the vertical edges `ey` back. */
pub(super) fn ratio_h(s: &Computed, w: i32, cw: i32, ey: i32) -> Option<i32> {
    let r = s.aspect.filter(|r| *r > 0.0)?;
    let base = if s.border_box { w } else { cw };
    let h = (base as f32 / r + 0.5) as i32;
    Some(if s.border_box { h } else { h + ey })
}

/* Whether a height taken from the aspect-ratio still grows to hold the
 * content: min-height:auto on a box with a ratio is its content height,
 * capped by max-height, unless the box is a scroll container (CSS Sizing
 * 4, automatic minimum size for boxes with a preferred aspect ratio).
 * Replaced elements size elsewhere and never get here. */
pub(super) fn content_floor(s: &Computed) -> bool {
    let scrolls = |o: Overflow| !matches!(o, Overflow::Visible | Overflow::Clip);
    s.min_height == Size::Auto && !scrolls(s.overflow_x) && !scrolls(s.overflow_y)
}
