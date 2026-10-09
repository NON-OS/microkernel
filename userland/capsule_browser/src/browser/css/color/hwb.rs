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

use super::args::{args, Args};
use super::rgbaf::Rgbaf;
use crate::browser::css::hsl_fn::hsl_rgb;

/// hwb(H W B): the pure hue mixed with W white and B black (percentages,
/// a bare number counting as one); W + B at or past 100% is a grey.
pub(super) fn parse_hwb(inner: &str) -> Option<u32> {
    let Args { c, alpha } = args(inner)?;
    let w = c[1].scaled(100.0) / 100.0;
    let b = c[2].scaled(100.0) / 100.0;
    Some(Rgbaf { rgb: hwb_rgb(c[0].scaled(0.0), w, b), a: alpha }.to_argb())
}

/// Hue degrees, whiteness and blackness 0..1 to rgb 0..1.
pub(super) fn hwb_rgb(h: f64, w: f64, b: f64) -> [f64; 3] {
    let (w, b) = (w.clamp(0.0, 1.0), b.clamp(0.0, 1.0));
    if w + b >= 1.0 {
        let grey = w / (w + b);
        return [grey; 3];
    }
    hsl_rgb(h, 1.0, 0.5).map(|v| v * (1.0 - w - b) + w)
}
