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

use super::super::fx::{rel, Clip, ClipR};
use super::fmath::isqrt;

/// The page rectangle [x0, y0, x1, y1] a clip-path leaves visible on a
/// border box at `b` ([x, y, w, h]): the bounding box of its shape. A
/// circle's length radius is measured against sqrt((w^2 + h^2) / 2), an
/// ellipse's against the width across and the height down; closest-side
/// and farthest-side measure from the centre to the box's edges.
pub(crate) fn clip_rect(clip: Clip, b: [i32; 4]) -> [i32; 4] {
    let [x, y, w, h] = b;
    match clip {
        Clip::Rect([l, t, r, btm]) => {
            [x + rel(l, w), y + rel(t, h), x + rel(r, w), y + rel(btm, h)]
        }
        Clip::Ellipse { cx, cy, rx, ry, circle } => {
            let (px, py) = (rel(cx, w), rel(cy, h));
            let near = [px.min(w - px), py.min(h - py)];
            let far = [px.max(w - px), py.max(h - py)];
            let diag = isqrt((w as i64 * w as i64 + h as i64 * h as i64) / 2) as i32;
            let radius = |r: ClipR, axis: usize| -> i32 {
                let (lo, hi) = if circle {
                    (near[0].min(near[1]), far[0].max(far[1]))
                } else {
                    (near[axis], far[axis])
                };
                let base = if circle { diag } else { [w, h][axis] };
                match r {
                    ClipR::Len(l) => rel(l, base),
                    ClipR::Closest => lo,
                    ClipR::Farthest => hi,
                }
                .max(0)
            };
            let (a, bb) = (radius(rx, 0), radius(ry, 1));
            [x + px - a, y + py - bb, x + px + a, y + py + bb]
        }
    }
}
