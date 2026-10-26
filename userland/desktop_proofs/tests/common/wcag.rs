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
//! WCAG 2.2's contrast ratio, written from the standard, for the proofs.

pub fn lum(c: u32) -> f64 {
    let ch = |s: u32| {
        let v = ((c >> s) & 0xFF) as f64 / 255.0;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * ch(16) + 0.7152 * ch(8) + 0.0722 * ch(0)
}

pub fn ratio(a: u32, b: u32) -> f64 {
    let (x, y) = (lum(a), lum(b));
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

/// `top`, with its alpha, over the opaque `under`.
pub fn over(under: u32, top: u32) -> u32 {
    let a = (top >> 24) & 0xFF;
    let mix = |s: u32| (((top >> s) & 0xFF) * a + ((under >> s) & 0xFF) * (255 - a)) / 255;
    0xFF00_0000 | (mix(16) << 16) | (mix(8) << 8) | mix(0)
}
