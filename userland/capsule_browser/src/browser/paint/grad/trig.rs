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

const PI: f32 = core::f32::consts::PI;

/* Unit vector of a CSS gradient angle: 0deg points up and angles grow
 * clockwise, so the vector is (sin a, -cos a). Quarter turns are exact, so
 * "to bottom" and "to right" run straight along an axis. */
pub(super) fn axis(deg: f32) -> (f32, f32) {
    let d = deg % 360.0;
    let d = if d < 0.0 { d + 360.0 } else { d };
    if d == 0.0 {
        (0.0, -1.0)
    } else if d == 90.0 {
        (1.0, 0.0)
    } else if d == 180.0 {
        (0.0, 1.0)
    } else if d == 270.0 {
        (-1.0, 0.0)
    } else {
        let r = d * PI / 180.0;
        (sin(r), -sin(r + PI / 2.0))
    }
}

/* Taylor sine through the x^9 term, on an argument first reduced to
 * [-pi, pi] and then folded into [-pi/2, pi/2] (sin(pi - t) = sin t), where
 * the series is good to about 4e-6. */
fn sin(x: f32) -> f32 {
    let mut t = x % (2.0 * PI);
    if t > PI {
        t -= 2.0 * PI;
    } else if t < -PI {
        t += 2.0 * PI;
    }
    if t > PI / 2.0 {
        t = PI - t;
    } else if t < -PI / 2.0 {
        t = -PI - t;
    }
    let t2 = t * t;
    t * (1.0 - t2 / 6.0 * (1.0 - t2 / 20.0 * (1.0 - t2 / 42.0 * (1.0 - t2 / 72.0))))
}
