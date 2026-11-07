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

//! The part of a window's buffer the frame clears before it draws. The
//! panel's ground is opaque and its straight rows overwrite their whole
//! span of the frame rect (fill_round), so clearing them first was a
//! store to every pixel the panel stores again. Only the shadow's margin and
//! the rows the rounded corners cross are cleared.

use super::palette::TRANSPARENT;
use super::rect::Rect;
use crate::paint::radius::clamp_radius;
use crate::paint::PaintBuffer;

/// Clear every pixel of `fb` that a panel of opaque `ground` over `f` with
/// corner radius `r` does not overwrite; a ground that is not opaque, or a
/// rect with no straight row, clears the whole buffer.
pub(super) fn clear_around(fb: &mut PaintBuffer, f: Rect, r: u32, ground: u32) {
    let (w, h) = (fb.width, fb.height);
    let r = clamp_radius(f.w, f.h, r);
    let top = f.y.saturating_add(r);
    let bottom = f.y.saturating_add(f.h).saturating_sub(r);
    let right = f.x.saturating_add(f.w);
    let whole = ground >> 24 != 0xFF || f.w == 0 || top >= bottom || bottom > h || right > w;
    if whole {
        fb.clear(TRANSPARENT);
        return;
    }
    fb.fill_rect(0, 0, w, top, TRANSPARENT);
    fb.fill_rect(0, bottom, w, h - bottom, TRANSPARENT);
    fb.fill_rect(0, top, f.x, bottom - top, TRANSPARENT);
    fb.fill_rect(right, top, w - right, bottom - top, TRANSPARENT);
}
