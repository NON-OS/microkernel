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
use super::powers::{cbrt, mul};
use super::rgbaf::Rgbaf;
use super::trig::sin_cos_deg;

/* OKLab (Ottosson 2020) against linear sRGB, by way of the cone space. */
const LIN_TO_LMS: [[f64; 3]; 3] = [
    [0.412_221_470_8, 0.536_332_536_3, 0.051_445_992_9],
    [0.211_903_498_2, 0.680_699_545_1, 0.107_396_956_6],
    [0.088_302_461_9, 0.281_718_837_6, 0.629_978_700_5],
];
const LMS_TO_LAB: [[f64; 3]; 3] = [
    [0.210_454_255_3, 0.793_617_785_0, -0.004_072_046_8],
    [1.977_998_495_1, -2.428_592_205_0, 0.450_593_709_9],
    [0.025_904_037_1, 0.782_771_766_2, -0.808_675_766_0],
];
const LAB_TO_LMS: [[f64; 3]; 3] = [
    [1.0, 0.396_337_777_4, 0.215_803_757_3],
    [1.0, -0.105_561_345_8, -0.063_854_172_8],
    [1.0, -0.089_484_177_5, -1.291_485_548_0],
];
const LMS_TO_LIN: [[f64; 3]; 3] = [
    [4.076_741_662_1, -3.307_711_591_3, 0.230_969_929_2],
    [-1.268_438_004_6, 2.609_757_401_1, -0.341_319_396_5],
    [-0.004_196_086_3, -0.703_418_614_7, 1.707_614_701_0],
];

pub(in crate::browser::css) fn oklab_to_linear(lab: [f64; 3]) -> [f64; 3] {
    let lms = mul(&LAB_TO_LMS, lab).map(|v| v * v * v);
    mul(&LMS_TO_LIN, lms)
}

pub(in crate::browser::css) fn linear_to_oklab(lin: [f64; 3]) -> [f64; 3] {
    mul(&LMS_TO_LAB, mul(&LIN_TO_LMS, lin).map(cbrt))
}

/// Polar (lightness, chroma, hue degrees) to rectangular (l, a, b).
pub(super) fn from_polar(l: f64, c: f64, h: f64) -> [f64; 3] {
    let (s, co) = sin_cos_deg(h);
    [l, c * co, c * s]
}

/// oklab(L a b) and oklch(L C H): L runs 0..1 (100% = 1), a, b and C take
/// 100% as 0.4, hue is in degrees.
pub(super) fn parse_oklab(inner: &str, polar: bool) -> Option<u32> {
    let Args { c, alpha } = args(inner)?;
    let l = c[0].scaled(1.0).clamp(0.0, 1.0);
    let lab = if polar {
        from_polar(l, c[1].scaled(0.4).max(0.0), c[2].scaled(0.0))
    } else {
        [l, c[1].scaled(0.4), c[2].scaled(0.4)]
    };
    Some(Rgbaf::from_linear(oklab_to_linear(lab), alpha).to_argb())
}
