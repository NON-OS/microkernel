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

use super::gauss::sqrt;

/// A rounded rectangle in screen px with corner radii top-left, top-right,
/// bottom-right, bottom-left.
#[derive(Clone, Copy)]
pub(super) struct RRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub r: [f32; 4],
}

impl RRect {
    /// Signed distance from the point to the edge, negative inside.
    pub(super) fn dist(&self, px: f32, py: f32) -> f32 {
        let (hw, hh) = (self.w / 2.0, self.h / 2.0);
        let (cx, cy) = (px - self.x - hw, py - self.y - hh);
        let r = match (cx >= 0.0, cy >= 0.0) {
            (false, false) => self.r[0],
            (true, false) => self.r[1],
            (true, true) => self.r[2],
            (false, true) => self.r[3],
        };
        let (qx, qy) = (cx.abs() - (hw - r), cy.abs() - (hh - r));
        let (ox, oy) = (qx.max(0.0), qy.max(0.0));
        sqrt(ox * ox + oy * oy) + qx.max(qy).min(0.0) - r
    }

    /// Moved by (dx, dy) and grown by `s` on every side (shrunk when
    /// negative); rounded corners grow with it, square ones stay square.
    pub(super) fn spread(&self, dx: f32, dy: f32, s: f32) -> RRect {
        let (w, h) = ((self.w + 2.0 * s).max(0.0), (self.h + 2.0 * s).max(0.0));
        let lim = w.min(h) / 2.0;
        let r = self.r.map(|r| if r > 0.0 { (r + s).clamp(0.0, lim) } else { 0.0 });
        RRect { x: self.x + dx - s, y: self.y + dy - s, w, h, r }
    }

    /// The largest radius on the top and on the bottom edge.
    pub(super) fn caps(&self) -> (f32, f32) {
        (self.r[0].max(self.r[1]), self.r[2].max(self.r[3]))
    }

    /// The box inside the border widths [top, right, bottom, left], its
    /// radii reduced by the thicker adjoining border; None when empty.
    pub(super) fn padding_box(&self, border: [u32; 4]) -> Option<RRect> {
        let [bt, br, bb, bl] = border.map(|b| b as f32);
        let edge = [bt.max(bl), bt.max(br), bb.max(br), bb.max(bl)];
        let (w, h) = (self.w - bl - br, self.h - bt - bb);
        let r = [0, 1, 2, 3].map(|i| (self.r[i] - edge[i]).max(0.0));
        (w > 0.0 && h > 0.0).then_some(RRect { x: self.x + bl, y: self.y + bt, w, h, r })
    }
}
