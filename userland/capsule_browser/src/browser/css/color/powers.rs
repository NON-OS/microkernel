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

use super::fmath::{exp, ln};

/// `x` to the power `y` for `x` >= 0; 0 to a positive power is 0.
pub(super) fn powf(x: f64, y: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    exp(y * ln(x))
}

/// Cube root keeping the sign, refined by one Newton step.
pub(super) fn cbrt(x: f64) -> f64 {
    if x == 0.0 || !x.is_finite() {
        return if x.is_finite() { 0.0 } else { x };
    }
    let r = exp(ln(x.abs()) / 3.0);
    let r = r - (r * r * r - x.abs()) / (3.0 * r * r);
    if x < 0.0 {
        -r
    } else {
        r
    }
}

/// Square root of a non-negative `x`; negatives give 0.
pub(super) fn sqrt(x: f64) -> f64 {
    if x <= 0.0 || !x.is_finite() {
        return if x.is_finite() { 0.0 } else { x.max(0.0) };
    }
    let r = exp(ln(x) / 2.0);
    (r + x / r) / 2.0
}

/// 3x3 matrix times column vector.
pub(super) fn mul(m: &[[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    let row = |r: &[f64; 3]| r[0] * v[0] + r[1] * v[1] + r[2] * v[2];
    [row(&m[0]), row(&m[1]), row(&m[2])]
}
