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

use super::bits::Bools;
use super::modes::{DC, HE, TM, VE};
use super::tables::bmode_probs;

/// The sixteen 4x4 modes of a macroblock in raster order, each coded
/// with probabilities chosen by the modes above and to its left.
pub(super) fn bmodes(br: &mut Bools, top: &mut [u8], left: &mut [u8; 4], out: &mut [u8; 16]) {
    for y in 0..4 {
        let mut m = left[y];
        for x in 0..4 {
            m = bmode(br, bmode_probs(top[x] as usize, m as usize));
            top[x] = m;
            out[y * 4 + x] = m;
        }
        left[y] = m;
    }
}

/// The chroma mode tree (RFC 6386 11.2).
pub(super) fn uv_mode(br: &mut Bools) -> u8 {
    if !br.bit(142) {
        DC
    } else if !br.bit(114) {
        VE
    } else if br.bit(183) {
        TM
    } else {
        HE
    }
}

/// The 4x4 mode tree (RFC 6386 11.2).
fn bmode(br: &mut Bools, p: &[u8]) -> u8 {
    const RD: u8 = 4;
    const VR: u8 = 5;
    const LD: u8 = 6;
    const VL: u8 = 7;
    const HD: u8 = 8;
    const HU: u8 = 9;
    match () {
        _ if !br.bit(p[0]) => DC,
        _ if !br.bit(p[1]) => TM,
        _ if !br.bit(p[2]) => VE,
        _ if !br.bit(p[3]) => {
            if !br.bit(p[4]) {
                HE
            } else if !br.bit(p[5]) {
                RD
            } else {
                VR
            }
        }
        _ if !br.bit(p[6]) => LD,
        _ if !br.bit(p[7]) => VL,
        _ if !br.bit(p[8]) => HD,
        _ => HU,
    }
}
