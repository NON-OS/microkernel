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

use super::huff::Coder;
use super::plane::Plane;
use crate::image::jpeg::bits::{extend, BitReader};
use crate::image::jpeg::coef::refuse::CORRUPT_DATA;
use crate::image::types::DecodeError;

pub fn clamp16(v: i32) -> i16 {
    v.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

/// A whole sequential block: the DC difference, then run/size coded AC
/// coefficients to the end of block.
pub fn seq_block(
    br: &mut BitReader,
    (dc, ac): (&Coder, &Coder),
    pred: &mut i32,
    p: &mut Plane,
    blk: usize,
) -> Result<(), DecodeError> {
    *pred = pred.wrapping_add(dc.value(br)?);
    p.set(blk, 0, clamp16(*pred));
    let mut k = 1;
    while k < 64 {
        let rs = ac.symbol(br)?;
        let (r, s) = ((rs >> 4) as usize, (rs & 15) as u32);
        if s == 0 {
            if r != 15 {
                break;
            }
            k += 16;
            continue;
        }
        k += r;
        if k > 63 {
            return Err(CORRUPT_DATA);
        }
        p.set(blk, k, clamp16(extend(br.read_bits(s)?, s)));
        k += 1;
    }
    Ok(())
}
