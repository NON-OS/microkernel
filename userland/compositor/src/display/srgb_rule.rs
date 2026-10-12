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

//! sRGB's transfer curve (IEC 61966-2-1): a level v in 0..1 is v / 12.92 of
//! linear light up to 0.04045, else ((v + 0.055) / 1.055) to the 2.4; and
//! back. Colour filters mix light, so they work on linear values. The
//! compositor has no libm, so each power is a Newton root raised to a whole
//! power: x^2.4 is (x^(1/5))^12, x^(1/2.4) is (x^(1/12))^5.

/// Linear light in twelve bits: fine enough that every byte level of sRGB
/// keeps a value of its own, dark ones included.
pub const LIN_MAX: u32 = 4095;

/// The `n`th root of `x` in 0..=1, by Newton's method from 1, which only
/// falls toward the root; 64 steps leave it far under one table level.
pub fn root(x: f32, n: u32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    let mut y = 1.0f32;
    for _ in 0..64 {
        let p = pow(y, n - 1);
        y = ((n - 1) as f32 * y + x / p) / n as f32;
    }
    y
}

pub fn pow(x: f32, n: u32) -> f32 {
    (0..n).fold(1.0, |a, _| a * x)
}

fn round(x: f32, max: u32) -> u32 {
    ((x.max(0.0) + 0.5) as u32).min(max)
}

/// A byte level of sRGB as linear light, 0..=LIN_MAX.
pub fn to_linear(v: u8) -> u16 {
    let s = v as f32 / 255.0;
    let lin = if s <= 0.04045 { s / 12.92 } else { pow(root((s + 0.055) / 1.055, 5), 12) };
    round(lin * LIN_MAX as f32, LIN_MAX) as u16
}

/// Linear light, 0..=LIN_MAX, as the nearest byte level of sRGB.
pub fn from_linear(l: u16) -> u8 {
    let x = l.min(LIN_MAX as u16) as f32 / LIN_MAX as f32;
    let s = if x <= 0.003_130_8 { x * 12.92 } else { 1.055 * pow(root(x, 12), 5) - 0.055 };
    round(s * 255.0, 255) as u8
}
