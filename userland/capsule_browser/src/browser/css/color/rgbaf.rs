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

use super::powers::powf;

/// A colour in gamma-encoded sRGB, each channel and alpha nominally 0..1;
/// channels may leave that range until the final clamp to 8 bits.
#[derive(Clone, Copy)]
pub(in crate::browser::css) struct Rgbaf {
    pub rgb: [f64; 3],
    pub a: f64,
}

impl Rgbaf {
    pub(in crate::browser::css) fn from_argb(c: u32) -> Self {
        let ch = |s: u32| ((c >> s) & 0xff) as f64 / 255.0;
        Rgbaf { rgb: [ch(16), ch(8), ch(0)], a: ch(24) }
    }

    /// Round each channel to 8 bits after clamping, alpha included.
    pub(in crate::browser::css) fn to_argb(self) -> u32 {
        let q = |v: f64| (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u32;
        let [r, g, b] = self.rgb;
        (q(self.a) << 24) | (q(r) << 16) | (q(g) << 8) | q(b)
    }

    pub(in crate::browser::css) fn from_linear(lin: [f64; 3], a: f64) -> Self {
        Rgbaf { rgb: lin.map(encode), a }
    }

    pub(in crate::browser::css) fn linear(self) -> [f64; 3] {
        self.rgb.map(decode)
    }
}

/// The sRGB transfer curve, odd-extended below zero.
pub(super) fn decode(c: f64) -> f64 {
    let m = c.abs();
    let v = if m <= 0.04045 { m / 12.92 } else { powf((m + 0.055) / 1.055, 2.4) };
    if c < 0.0 {
        -v
    } else {
        v
    }
}

/// Inverse of `decode`: linear light back to the sRGB curve.
pub(super) fn encode(c: f64) -> f64 {
    let m = c.abs();
    let v = if m <= 0.003_130_8 { m * 12.92 } else { 1.055 * powf(m, 1.0 / 2.4) - 0.055 };
    if c < 0.0 {
        -v
    } else {
        v
    }
}
