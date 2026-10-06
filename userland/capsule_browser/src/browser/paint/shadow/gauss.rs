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

/* Newton square root from a halved-exponent guess; core has none. */
pub(super) fn sqrt(x: f32) -> f32 {
    if !(x > 0.0) {
        return 0.0;
    }
    let mut y = f32::from_bits((x.to_bits() >> 1) + 0x1fc0_0000);
    for _ in 0..3 {
        y = 0.5 * (y + x / y);
    }
    y
}

/// Coverage 0..1 of a shape blurred by a Gaussian of radius `b` (sigma
/// b/2), at signed distance `d` from its edge; unblurred edges antialias
/// over one pixel. erf is Abramowitz-Stegun 7.1.27, within 5e-4.
pub(super) fn blurred(d: f32, b: f32) -> f32 {
    if b < 0.5 {
        return (0.5 - d).clamp(0.0, 1.0);
    }
    let z = -d / (b / 2.0);
    let x = z.abs() / core::f32::consts::SQRT_2;
    let t = 1.0 + x * (0.278_393 + x * (0.230_389 + x * (0.000_972 + x * 0.078_108)));
    let t2 = t * t;
    let erf = 1.0 - 1.0 / (t2 * t2);
    if z >= 0.0 {
        0.5 + 0.5 * erf
    } else {
        0.5 - 0.5 * erf
    }
}
