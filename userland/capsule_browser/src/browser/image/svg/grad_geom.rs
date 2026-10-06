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

use super::grad_paint::GradPaint;
use super::math::sqrt;

impl GradPaint {
    pub(super) fn linear_t(&self, p: [f32; 2]) -> f32 {
        let [x1, y1, x2, y2, _] = self.geom;
        let (dx, dy) = (x2 - x1, y2 - y1);
        let len2 = dx * dx + dy * dy;
        if len2 <= 0.0 {
            return 1.0;
        }
        ((p[0] - x1) * dx + (p[1] - y1) * dy) / len2
    }

    /* The circle through p of the family from the focal point (radius 0)
     * to the end circle (radius r): solve |p - f - t d| = t r for t >= 0,
     * d = c - f, the focal point kept inside the end circle. */
    pub(super) fn radial_t(&self, p: [f32; 2]) -> f32 {
        let [cx, cy, r, fx, fy] = self.geom;
        let (dx, dy, px, py) = (cx - fx, cy - fy, p[0] - fx, p[1] - fy);
        let a = dx * dx + dy * dy - r * r;
        if r <= 0.0 || a >= 0.0 {
            return 1.0;
        }
        let b = px * dx + py * dy;
        (b - sqrt((b * b - a * (px * px + py * py)).max(0.0))) / a
    }
}
