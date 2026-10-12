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

//! Night light: a colour temperature in kelvin turned into three ramps,
//! one per channel, that every composed pixel is looked up through. Red is
//! kept and green and blue fall as the light warms, after the colour of a
//! black body at that temperature as the commonly reproduced sRGB tables
//! give it (approximate, rounded to whole levels; unverified against a
//! colorimetric source). 6500 K and above is no change. Pure, so the host
//! proofs pin that every ramp only ever rises.

/// The coolest light is no change; the warmest the shell may ask for.
pub const NEUTRAL_K: u32 = 6500;
pub const WARMEST_K: u32 = 1000;

/// (kelvin, green, blue) out of 255, every 500 K, warmest first.
const BODY: [(u32, u32, u32); 12] = [
    (1000, 56, 0),
    (1500, 109, 0),
    (2000, 137, 18),
    (2500, 161, 72),
    (3000, 180, 107),
    (3500, 196, 137),
    (4000, 209, 163),
    (4500, 219, 186),
    (5000, 228, 206),
    (5500, 236, 224),
    (6000, 246, 237),
    (6500, 255, 255),
];

/// Red, green and blue gains out of 255 at `kelvin`, between table rows
/// by straight line.
pub fn gains(kelvin: u32) -> [u32; 3] {
    let k = kelvin.clamp(WARMEST_K, NEUTRAL_K);
    let i = BODY.iter().rposition(|r| r.0 <= k).unwrap_or(0);
    let (lo, hi) = (BODY[i], BODY[(i + 1).min(BODY.len() - 1)]);
    let span = (hi.0 - lo.0).max(1);
    let mix = |a: u32, b: u32| (a * (span - (k - lo.0)) + b * (k - lo.0)) / span;
    [255, mix(lo.1, hi.1), mix(lo.2, hi.2)]
}

/// The three lookup ramps: level v of a channel becomes v times its gain.
pub fn ramps(kelvin: u32) -> [[u8; 256]; 3] {
    let g = gains(kelvin);
    let mut r = [[0u8; 256]; 3];
    for (c, ramp) in r.iter_mut().enumerate() {
        for (v, out) in ramp.iter_mut().enumerate() {
            *out = (v as u32 * g[c] / 255) as u8;
        }
    }
    r
}

/// Whether `kelvin` changes anything.
pub fn active(kelvin: u32) -> bool {
    kelvin != 0 && kelvin < NEUTRAL_K
}
