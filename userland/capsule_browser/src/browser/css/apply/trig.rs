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

use core::f32::consts::{FRAC_PI_2, PI};

/* Sine by reduction to [-pi/2, pi/2] and a degree-9 Taylor polynomial; the
 * error stays under 4e-6, well below a pixel on any box. */
pub(super) fn sin(x: f32) -> f32 {
    let mut t = x % (2.0 * PI);
    if t > PI {
        t -= 2.0 * PI;
    } else if t < -PI {
        t += 2.0 * PI;
    }
    if t > FRAC_PI_2 {
        t = PI - t;
    } else if t < -FRAC_PI_2 {
        t = -PI - t;
    }
    let t2 = t * t;
    t * (1.0 - t2 / 6.0 * (1.0 - t2 / 20.0 * (1.0 - t2 / 42.0 * (1.0 - t2 / 72.0))))
}

pub(super) fn cos(x: f32) -> f32 {
    sin(x + FRAC_PI_2)
}

/// Tangent for skew angles; a right angle has none and skews to nothing.
pub(super) fn tan(x: f32) -> f32 {
    let c = cos(x);
    if c.abs() < 1e-6 {
        0.0
    } else {
        sin(x) / c
    }
}
