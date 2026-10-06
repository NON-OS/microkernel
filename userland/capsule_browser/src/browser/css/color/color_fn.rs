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
use super::lab::D50_TO_LIN;
use super::powers::{mul, powf};
use super::rgbaf::{decode, Rgbaf};

const P3_TO_LIN: [[f64; 3]; 3] = [
    [1.224_940_176_280_56, -0.224_940_176_280_56, 0.0],
    [-0.042_056_954_709_688_156, 1.042_056_954_709_688_3, 0.0],
    [-0.019_637_554_590_334_394, -0.078_636_045_550_631_82, 1.098_273_600_140_966_5],
];
const REC2020_TO_LIN: [[f64; 3]; 3] = [
    [1.660_491_002_108_435_4, -0.587_641_138_788_549_4, -0.072_849_863_319_884_8],
    [-0.124_550_474_521_590_72, 1.132_899_897_125_959_6, -0.008_349_422_604_369_506],
    [-0.018_150_763_354_905_262, -0.100_578_898_008_007_37, 1.118_729_661_362_913],
];
const D65_TO_LIN: [[f64; 3]; 3] = [
    [3.240_969_941_904_522_6, -1.537_383_177_570_094, -0.498_610_760_293_003_4],
    [-0.969_243_636_280_879_6, 1.875_967_501_507_720_2, 0.041_555_057_407_175_59],
    [0.055_630_079_696_993_66, -0.203_976_958_888_976_52, 1.056_971_514_242_878_6],
];

/// color(<space> c1 c2 c3 [/ a]) for srgb, srgb-linear, display-p3,
/// rec2020 and xyz (d50, d65); each channel takes 100% as 1.
pub(super) fn parse_color_fn(inner: &str) -> Option<u32> {
    let inner = inner.trim_start();
    let cut = inner.find(|c: char| c.is_ascii_whitespace())?;
    let (space, rest) = inner.split_at(cut);
    let Args { c, alpha } = args(rest)?;
    let v = c.map(|x| x.scaled(1.0));
    let lin = match space.to_ascii_lowercase().as_str() {
        "srgb" => v.map(decode),
        "srgb-linear" => v,
        "display-p3" => mul(&P3_TO_LIN, v.map(decode)),
        "rec2020" => mul(&REC2020_TO_LIN, v.map(rec2020_decode)),
        "xyz" | "xyz-d65" => mul(&D65_TO_LIN, v),
        "xyz-d50" => mul(&D50_TO_LIN, v),
        _ => return None,
    };
    Some(Rgbaf::from_linear(lin, alpha).to_argb())
}

/// The Rec. 2020 transfer curve, odd-extended below zero.
fn rec2020_decode(c: f64) -> f64 {
    const A: f64 = 1.099_296_826_809_44;
    const B: f64 = 0.018_053_968_510_807;
    let m = c.abs();
    let v = if m < B * 4.5 { m / 4.5 } else { powf((m + A - 1.0) / A, 1.0 / 0.45) };
    if c < 0.0 {
        -v
    } else {
        v
    }
}
