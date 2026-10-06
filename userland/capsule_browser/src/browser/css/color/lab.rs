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
use super::oklab::from_polar;
use super::powers::{cbrt, mul};
use super::rgbaf::Rgbaf;

/* CIE Lab on the D50 white, as CSS defines it; XYZ-D50 reaches linear sRGB
 * through the Bradford adaptation folded into one matrix. */
pub(super) const D50_TO_LIN: [[f64; 3]; 3] = [
    [3.134_135_852_900_118_6, -1.617_385_998_018_043_2, -0.490_662_217_911_097_4],
    [-0.978_795_476_555_777_5, 1.916_254_377_395_988, 0.033_442_873_390_366_88],
    [0.071_955_392_557_947_42, -0.228_976_759_815_181_98, 1.405_386_035_113_118],
];
const LIN_TO_D50: [[f64; 3]; 3] = [
    [0.436_065_746_874_269_2, 0.385_151_509_590_159_8, 0.143_078_419_965_138_62],
    [0.222_493_177_110_565_1, 0.716_887_013_094_482_7, 0.060_619_809_794_952_32],
    [0.013_923_921_463_169_356, 0.097_081_324_231_410_14, 0.714_099_356_815_880_8],
];
const WHITE: [f64; 3] = [0.3457 / 0.3585, 1.0, (1.0 - 0.3457 - 0.3585) / 0.3585];
const EPS: f64 = 216.0 / 24389.0;
const KAPPA: f64 = 24389.0 / 27.0;

pub(in crate::browser::css) fn lab_to_linear(lab: [f64; 3]) -> [f64; 3] {
    let fy = (lab[0] + 16.0) / 116.0;
    let f = [lab[1] / 500.0 + fy, fy, fy - lab[2] / 200.0];
    let inv = |t: f64| if t * t * t > EPS { t * t * t } else { (116.0 * t - 16.0) / KAPPA };
    let y = if lab[0] > KAPPA * EPS { fy * fy * fy } else { lab[0] / KAPPA };
    let xyz = [inv(f[0]) * WHITE[0], y, inv(f[2]) * WHITE[2]];
    mul(&D50_TO_LIN, xyz)
}

pub(in crate::browser::css) fn linear_to_lab(lin: [f64; 3]) -> [f64; 3] {
    let xyz = mul(&LIN_TO_D50, lin);
    let fwd = |v: f64| if v > EPS { cbrt(v) } else { (KAPPA * v + 16.0) / 116.0 };
    let f = [fwd(xyz[0] / WHITE[0]), fwd(xyz[1]), fwd(xyz[2] / WHITE[2])];
    [116.0 * f[1] - 16.0, 500.0 * (f[0] - f[1]), 200.0 * (f[1] - f[2])]
}

/// lab(L a b) and lch(L C H): L runs 0..100, a and b take 100% as 125,
/// C takes 100% as 150, hue is in degrees.
pub(super) fn parse_lab(inner: &str, polar: bool) -> Option<u32> {
    let Args { c, alpha } = args(inner)?;
    let l = c[0].scaled(100.0).clamp(0.0, 100.0);
    let lab = if polar {
        from_polar(l, c[1].scaled(150.0).max(0.0), c[2].scaled(0.0))
    } else {
        [l, c[1].scaled(125.0), c[2].scaled(125.0)]
    };
    Some(Rgbaf::from_linear(lab_to_linear(lab), alpha).to_argb())
}
