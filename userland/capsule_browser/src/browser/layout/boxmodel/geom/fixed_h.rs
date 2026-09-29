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

use crate::browser::css::{Computed, Size};

/* A CSS height as a border-box height: under content-box (the default) the
 * vertical padding and border `ey` go on top. A percentage needs a
 * definite containing height `cb_h`. */
fn border_box_h(s: &Computed, v: Size, cb_h: Option<i32>, ey: i32) -> Option<i32> {
    let v = v.definite_px().or_else(|| v.resolve(cb_h?))?;
    Some(if s.border_box { v } else { v + ey })
}

/* A definite border-box height, or None when the box sizes to its content.
 * A pixel or calc height is always definite. A percentage height resolves
 * only when the containing block has a definite height `cb_h`; otherwise the
 * box falls back to content sizing, as CSS specifies. */
fn fixed_h(s: &Computed, cb_h: Option<i32>, ey: i32) -> Option<i32> {
    border_box_h(s, s.height, cb_h, ey)
}

/* aspect-ratio: an auto height follows the border-box width `w` (content
 * width `cw`). The ratio applies to the box box-sizing names, so a
 * content-box ratio adds the vertical edges `ey` back. */
fn ratio_h(s: &Computed, w: i32, cw: i32, ey: i32) -> Option<i32> {
    let r = s.aspect.filter(|r| *r > 0.0)?;
    let base = if s.border_box { w } else { cw };
    let h = (base as f32 / r + 0.5) as i32;
    Some(if s.border_box { h } else { h + ey })
}

/* Clamp a border-box height `h` between min-height and max-height, both
 * read like the height itself. */
pub(crate) fn min_max_h(s: &Computed, h: i32, cb_h: Option<i32>, ey: i32) -> i32 {
    let mut h = h;
    if let Some(mx) = border_box_h(s, s.max_height, cb_h, ey) {
        h = h.min(mx);
    }
    if let Some(mn) = border_box_h(s, s.min_height, cb_h, ey) {
        h = h.max(mn);
    }
    h.max(0)
}

/// The definite border-box height of a box `w` wide (content `cw`, vertical
/// edges `ey`): the height its insets pinned, else its CSS height, else one
/// its aspect-ratio gives, clamped by min-height and max-height. None when
/// its content decides.
pub(crate) fn def_h(s: &Computed, pin: Option<i32>, cb_h: Option<i32>, g: [i32; 3]) -> Option<i32> {
    let [w, cw, ey] = g;
    let h = pin.or_else(|| fixed_h(s, cb_h, ey)).or_else(|| ratio_h(s, w, cw, ey))?;
    Some(min_max_h(s, h, cb_h, ey))
}
