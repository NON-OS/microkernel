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

/* hue-rotate: the Filter Effects 1 matrix for an angle, and the angle. */

use super::filter_mats::mat;

pub(super) fn hue(rad: f32) -> [f32; 13] {
    let (c, s) = (super::trig::cos(rad), super::trig::sin(rad));
    mat([
        [
            0.213 + c * 0.787 - s * 0.213,
            0.715 - c * 0.715 - s * 0.715,
            0.072 - c * 0.072 + s * 0.928,
        ],
        [
            0.213 - c * 0.213 + s * 0.143,
            0.715 + c * 0.285 + s * 0.140,
            0.072 - c * 0.072 - s * 0.283,
        ],
        [
            0.213 - c * 0.213 - s * 0.787,
            0.715 - c * 0.715 + s * 0.715,
            0.072 + c * 0.928 + s * 0.072,
        ],
    ])
}

/// A hue-rotate angle in radians: deg, rad or turn, bare 0 allowed.
pub(super) fn angle(arg: &str) -> Option<f32> {
    let unit = |u: &str| arg.strip_suffix(u).and_then(|n| n.trim().parse::<f32>().ok());
    unit("deg")
        .map(|d| d.to_radians())
        .or_else(|| unit("turn").map(|t| t * core::f32::consts::TAU))
        .or_else(|| unit("rad"))
        .or_else(|| (arg.trim() == "0").then_some(0.))
}
