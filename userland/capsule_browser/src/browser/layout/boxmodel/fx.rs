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

use super::affine::Affine;

/// A box-relative length: px plus per-mille of the box's own width (for x)
/// or height (for y), resolved once layout knows the border box.
pub type Rel = (i32, i32);

/// Resolve a box-relative length against a box side of `base` px.
pub fn rel(r: Rel, base: i32) -> i32 {
    r.0.saturating_add((base as i64 * r.1 as i64 / 1000).clamp(-1 << 30, 1 << 30) as i32)
}

/// A clip-path radius: a length, or a keyword measured from the centre.
#[derive(Clone, Copy)]
pub enum ClipR {
    /// px plus per-mille of the reference: the width for rx, the height for
    /// ry, and sqrt((w^2 + h^2) / 2) for a circle.
    Len(Rel),
    Closest,
    Farthest,
}

/// clip-path, reduced to what paints: everything outside the shape's
/// bounding rectangle is clipped away.
#[derive(Clone, Copy)]
pub enum Clip {
    /// Visible x0, y0, x1, y1 relative to the border box's top-left corner.
    Rect([Rel; 4]),
    /// circle() (rx and ry are the same radius) or ellipse().
    Ellipse { cx: Rel, cy: Rel, rx: ClipR, ry: ClipR, circle: bool },
}

/// Effects applied to a box after it has been laid out: its transform about
/// transform-origin, and its clip-path. Neither moves the box in flow.
#[derive(Clone, Copy)]
pub struct Fx {
    pub transform: Option<Affine>,
    pub origin: [Rel; 2],
    pub clip: Option<Clip>,
    /* The background image slot holds a mask-image: its alpha shows the
     * background color, which paints nowhere else. */
    pub mask: bool,
}

impl Fx {
    /// No transform, the default 50% 50% origin, no clip, no mask.
    pub const NONE: Fx =
        Fx { transform: None, origin: [(0, 500), (0, 500)], clip: None, mask: false };
}
