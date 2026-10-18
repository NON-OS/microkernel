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

//! What an output's settings do to each composed pixel, in this order: the
//! filter's matrix on linear light, then night light's ramps, then high
//! contrast's curve, both on the byte levels. Night light (the shell's
//! colour temperature) applies to every output; the rest per output.

use super::access_rule::{Access, FILTER_DEUTAN, FILTER_GREYSCALE, FILTER_PROTAN, FILTER_TRITAN};
use super::cvd_filter::{correct, Cvd};
use super::m3::M3;

/// Relative luminance from linear sRGB, ITU-R BT.709's weights.
pub const LUMA: [f32; 3] = [0.2126, 0.7152, 0.0722];

/// The filter's matrix on linear RGB; None for no filter.
pub fn matrix(filter: u32) -> Option<M3> {
    match filter {
        FILTER_GREYSCALE => Some([LUMA; 3]),
        FILTER_PROTAN => correct(Cvd::Protan),
        FILTER_DEUTAN => correct(Cvd::Deutan),
        FILTER_TRITAN => correct(Cvd::Tritan),
        _ => None,
    }
}

/// High contrast: levels pushed 1.75 times as far from mid grey, the ends
/// held, so text and edges stand out and nothing turns over.
pub fn contrast(v: u8) -> u8 {
    (128 + (v as i32 - 128) * 7 / 4).clamp(0, 255) as u8
}

/// The per-channel ramps after the matrix: night light's, then contrast;
/// None when they change nothing.
pub fn ramps(a: &Access, night: Option<&[[u8; 256]; 3]>) -> Option<[[u8; 256]; 3]> {
    if !a.contrast && night.is_none() {
        return None;
    }
    let mut r = [[0u8; 256]; 3];
    for (c, ramp) in r.iter_mut().enumerate() {
        for (v, out) in ramp.iter_mut().enumerate() {
            let lit = night.map_or(v as u8, |n| n[c][v]);
            *out = if a.contrast { contrast(lit) } else { lit };
        }
    }
    Some(r)
}
