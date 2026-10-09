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

use super::blocks::clamp16;
use super::huff::Coder;
use super::plane::Plane;
use crate::image::jpeg::bits::BitReader;
use crate::image::types::DecodeError;

/// First DC scan of a progressive frame: the difference, scaled by 2^al.
pub fn dc_first(
    br: &mut BitReader,
    dc: &Coder,
    pred: &mut i32,
    al: u32,
    p: &mut Plane,
    blk: usize,
) -> Result<(), DecodeError> {
    *pred = pred.wrapping_add(dc.value(br)?);
    p.set(blk, 0, clamp16(pred.wrapping_shl(al)));
    Ok(())
}

/// A DC refinement scan: one more bit of precision at position al.
pub fn dc_refine(
    br: &mut BitReader,
    al: u32,
    p: &mut Plane,
    blk: usize,
) -> Result<(), DecodeError> {
    if br.read_bits(1)? == 1 {
        p.set(blk, 0, p.get(blk, 0) | (1i32 << al) as i16);
    }
    Ok(())
}
