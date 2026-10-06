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

use core::f64::consts::PI;

use super::fmath::floor;
use super::powers::sqrt;

/// A hue in degrees brought into [0, 360).
pub(in crate::browser::css) fn wrap_deg(h: f64) -> f64 {
    /* Past 2^50 degrees the fraction of a turn is below f64 precision. */
    if !h.is_finite() || h.abs() > 1e15 {
        return 0.0;
    }
    h - 360.0 * floor(h / 360.0)
}

/// (sin, cos) of an angle in degrees. The angle is shifted by half a turn
/// into [-pi, pi), where the Taylor series to x^23 is exact in f64.
pub(super) fn sin_cos_deg(deg: f64) -> (f64, f64) {
    let x = (wrap_deg(deg) - 180.0) * PI / 180.0;
    let x2 = x * x;
    let (mut s, mut c) = (0.0, 0.0);
    let (mut ts, mut tc) = (x, 1.0);
    for k in 0..12 {
        s += ts;
        c += tc;
        let n = (2 * k + 2) as f64;
        ts *= -x2 / (n * (n + 1.0));
        tc *= -x2 / ((n - 1.0) * n);
    }
    /* sin(x + pi) = -sin(x), cos(x + pi) = -cos(x). */
    (-s, -c)
}

/// atan2(y, x) in degrees within [0, 360); 0 for the origin.
pub(super) fn atan2_deg(y: f64, x: f64) -> f64 {
    if x == 0.0 && y == 0.0 {
        return 0.0;
    }
    let (ax, ay) = (x.abs(), y.abs());
    let z = if ay <= ax { ay / ax } else { ax / ay };
    /* Two half-angle steps take z below 0.2 for a short odd series. */
    let h1 = z / (1.0 + sqrt(1.0 + z * z));
    let h2 = h1 / (1.0 + sqrt(1.0 + h1 * h1));
    let (t2, mut term, mut sum) = (h2 * h2, h2, 0.0);
    for k in 0..12 {
        sum += term / (2 * k + 1) as f64;
        term *= -t2;
    }
    let mut a = 4.0 * sum;
    if ay > ax {
        a = PI / 2.0 - a;
    }
    if x < 0.0 {
        a = PI - a;
    }
    let deg = a * 180.0 / PI;
    wrap_deg(if y < 0.0 { -deg } else { deg })
}
