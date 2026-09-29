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

/* Integer square root by Newton's method, floor of the root. */
pub(crate) fn isqrt(v: i64) -> i64 {
    if v < 2 {
        return v.max(0);
    }
    let mut r = v;
    let mut next = (r + 1) / 2;
    while next < r {
        r = next;
        next = (r + v / r) / 2;
    }
    r
}

/* Square root by Newton's method from a power-of-two guess; core has no
 * float sqrt without the standard library. */
pub(crate) fn sqrt(v: f32) -> f32 {
    if v <= 0.0 || !v.is_finite() {
        return 0.0;
    }
    let mut r = f32::from_bits((v.to_bits() >> 1) + 0x1fc0_0000);
    for _ in 0..4 {
        r = 0.5 * (r + v / r);
    }
    r
}

/* Nearest whole number, halves away from zero; core has no float round. */
pub(crate) fn round(v: f32) -> i32 {
    if v < 0.0 {
        (v - 0.5) as i32
    } else {
        (v + 0.5) as i32
    }
}
