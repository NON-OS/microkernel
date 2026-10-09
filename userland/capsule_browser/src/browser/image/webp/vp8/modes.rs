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
use super::bmode_tree::{bmodes, uv_mode};

/* Prediction modes as the 4x4 mode tree numbers them; the 16x16 and
 * chroma modes reuse DC, TM, VE (vertical) and HE (horizontal). */
pub(super) const DC: u8 = 0;
pub(super) const TM: u8 = 1;
pub(super) const VE: u8 = 2;
pub(super) const HE: u8 = 3;

/// One macroblock's header from partition 0.
#[derive(Clone, Copy, Default)]
pub(super) struct Mb {
    pub seg: usize,
    pub skip: bool,
    pub i4x4: bool,
    pub ymode: u8,
    pub bmodes: [u8; 16],
    pub uvmode: u8,
}

/// Read a macroblock's segment, skip flag and prediction modes (RFC 6386
/// 11). `top` holds the 4x4 modes along the bottom of the macroblock above
/// and `left` those along the right of the one before; a 16x16 mode sets
/// them as its 4x4 equivalent.
pub(super) fn parse_mb(
    br: &mut Bools,
    seg: Option<[u8; 3]>,
    skip: Option<u8>,
    top: &mut [u8],
    left: &mut [u8; 4],
) -> Mb {
    let mut mb = Mb::default();
    if let Some(p) = seg {
        mb.seg = if !br.bit(p[0]) { br.bit(p[1]) as usize } else { 2 + br.bit(p[2]) as usize };
    }
    mb.skip = skip.is_some_and(|p| br.bit(p));
    mb.i4x4 = !br.bit(145);
    if !mb.i4x4 {
        mb.ymode = if br.bit(156) {
            [HE, TM][br.bit(128) as usize]
        } else {
            [DC, VE][br.bit(163) as usize]
        };
        top.fill(mb.ymode);
        left.fill(mb.ymode);
    } else {
        bmodes(br, top, left, &mut mb.bmodes);
    }
    mb.uvmode = uv_mode(br);
    mb
}
