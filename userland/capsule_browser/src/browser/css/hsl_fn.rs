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

use super::color::args::{args, Args};
use super::color::rgbaf::Rgbaf;
use super::color::trig::wrap_deg;

/* hsl()/hsla() arguments: hue in degrees (or another angle unit),
 * saturation and lightness as percentages (a bare number counts as one),
 * and an optional alpha that is kept. */
pub(super) fn parse_hsl(inner: &str) -> Option<u32> {
    let Args { c, alpha } = args(inner)?;
    let s = c[1].scaled(100.0) / 100.0;
    let l = c[2].scaled(100.0) / 100.0;
    let rgb = hsl_rgb(c[0].scaled(0.0), s, l);
    Some(Rgbaf { rgb, a: alpha }.to_argb())
}

/// CSS Color 4 hsl-to-rgb: hue in degrees, s and l 0..1, channels 0..1.
pub(super) fn hsl_rgb(h: f64, s: f64, l: f64) -> [f64; 3] {
    let (s, l) = (s.clamp(0.0, 1.0), l.clamp(0.0, 1.0));
    let h = wrap_deg(h);
    let f = |n: f64| {
        let k = n + h / 30.0;
        let k = if k >= 12.0 { k - 12.0 } else { k };
        let a = s * l.min(1.0 - l);
        l - a * (k - 3.0).min(9.0 - k).clamp(-1.0, 1.0)
    };
    [f(0.0), f(8.0), f(4.0)]
}

/// rgb 0..1 to (hue degrees, s, l); an achromatic colour reports hue None.
pub(super) fn rgb_hsl(rgb: [f64; 3]) -> (Option<f64>, f64, f64) {
    let [r, g, b] = rgb;
    let (max, min) = (r.max(g).max(b), r.min(g).min(b));
    let (d, l) = (max - min, (max + min) / 2.0);
    if d.abs() < 1e-9 {
        return (None, 0.0, l);
    }
    let s = if l <= 0.0 || l >= 1.0 { 0.0 } else { (max - l) / l.min(1.0 - l) };
    let h = if max == r {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    (Some(h * 60.0), s, l)
}
