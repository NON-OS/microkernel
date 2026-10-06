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

use alloc::vec::Vec;

use super::affine::Affine;
use super::math::{cos, sin, sqrt, PI};

type P = [f32; 2];

pub(super) fn dist(a: P, b: P) -> f32 {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    sqrt(dx * dx + dy * dy)
}

/// The left normal of a to b, `hw` long.
pub(super) fn normal(a: P, b: P, hw: f32) -> P {
    let d = dist(a, b);
    [-(b[1] - a[1]) / d * hw, (b[0] - a[0]) / d * hw]
}

/// `poly` wound one fixed way (non-negative signed area), so the nonzero
/// rule unions every stroke piece.
pub(super) fn turned(mut poly: Vec<P>) -> Vec<P> {
    let n = poly.len();
    let cross = |i: usize| poly[i][0] * poly[(i + 1) % n][1] - poly[(i + 1) % n][0] * poly[i][1];
    if (0..n).map(cross).sum::<f32>() < 0.0 {
        poly.reverse();
    }
    poly
}

/// The quad from a to b reaching `n` to either side.
pub(super) fn band(a: P, b: P, n: P) -> Vec<P> {
    let at = |p: P, k: f32| [p[0] + k * n[0], p[1] + k * n[1]];
    turned([at(a, 1.0), at(b, 1.0), at(b, -1.0), at(a, -1.0)].to_vec())
}

/// A circle of radius `r` around `c`, with more sides the larger it is.
pub(super) fn disc(c: P, r: f32) -> Vec<P> {
    let n = ((r * 2.0) as usize).clamp(8, 48);
    let at = |i: usize| 2.0 * PI * i as f32 / n as f32;
    turned((0..n).map(|i| [c[0] + r * cos(at(i)), c[1] + r * sin(at(i))]).collect())
}

/// Subpaths mapped from user to device space.
pub(super) fn device(polys: &[Vec<P>], t: &Affine) -> Vec<Vec<P>> {
    polys.iter().map(|sp| sp.iter().map(|&pt| t.apply(pt)).collect()).collect()
}

/// The user-space bounding box [x0, y0, x1, y1] of some subpaths.
pub(super) fn bbox(polys: &[Vec<P>]) -> [f32; 4] {
    let mut b = [f32::MAX, f32::MAX, f32::MIN, f32::MIN];
    for p in polys.iter().flatten() {
        b = [b[0].min(p[0]), b[1].min(p[1]), b[2].max(p[0]), b[3].max(p[1])];
    }
    b
}
