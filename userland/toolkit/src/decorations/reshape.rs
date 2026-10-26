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
//! The frame's measures at one to one (the design notes, Window
//! The rect a drag of a window's edges leaves: the grabbed edges move by the
//! pointer's travel, the others stay where they were, and no side passes
//! the least size or the top of the work area.

use super::edges::Edges;

/// A window's origin and size.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Placed {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// `base` dragged by (`dx`, `dy`) on `edges`, no smaller than `least`
/// and with its top no higher than `top`.
pub fn reshaped(
    base: Placed,
    edges: Edges,
    (dx, dy): (i64, i64),
    least: (u32, u32),
    top: u32,
) -> Placed {
    let (x0, y0, w0, h0) = (base.x as i64, base.y as i64, base.w as i64, base.h as i64);
    let (lw, lh) = (least.0 as i64, least.1 as i64);
    let (mut x, mut w) = (x0, w0);
    if edges.right {
        w = (w0 + dx).max(lw);
    } else if edges.left {
        w = (w0 - dx).max(lw).min(x0 + w0);
        x = x0 + w0 - w;
    }
    let (mut y, mut h) = (y0, h0);
    if edges.bottom {
        h = (h0 + dy).max(lh);
    } else if edges.top {
        h = (h0 - dy).max(lh).min(y0 + h0 - top as i64);
        y = y0 + h0 - h;
    }
    let u = |v: i64| v.clamp(0, u32::MAX as i64) as u32;
    Placed { x: u(x), y: u(y), w: u(w), h: u(h) }
}

/// `p` at size `w` by `h` (the size a window was settled to after the drag)
/// with the edges that did not move from `base` kept where they were: a
/// window placed from its left or top keeps its right or bottom.
pub fn keep_far_edges(p: Placed, base: Placed, w: u32, h: u32) -> Placed {
    let x = if p.x != base.x { (base.x + base.w).saturating_sub(w) } else { p.x };
    let y = if p.y != base.y { (base.y + base.h).saturating_sub(h) } else { p.y };
    Placed { x, y, w, h }
}
