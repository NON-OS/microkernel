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
use crate::image::jpeg::bits::{extend, BitReader};
use crate::image::jpeg::coef::refuse::CORRUPT_DATA;
use crate::image::types::DecodeError;

/// The spectral band `ss..=se` coded by one AC scan and its bit position.
#[derive(Clone, Copy)]
pub struct Band {
    pub ss: usize,
    pub se: usize,
    pub al: u32,
}

/// First AC scan of a band: run/size coded values scaled by 2^al, with
/// end-of-band runs (`eobrun`) that skip whole blocks.
pub fn ac_first(
    br: &mut BitReader,
    ac: &Coder,
    b: Band,
    eobrun: &mut u32,
    p: &mut Plane,
    blk: usize,
) -> Result<(), DecodeError> {
    if *eobrun > 0 {
        *eobrun -= 1;
        return Ok(());
    }
    let mut k = b.ss;
    while k <= b.se {
        let rs = ac.symbol(br)?;
        let (r, s) = ((rs >> 4) as u32, (rs & 15) as u32);
        if s == 0 {
            if r < 15 {
                *eobrun = (1 << r) - 1 + br.read_bits(r)?;
                break;
            }
            k += 16;
            continue;
        }
        k += r as usize;
        if k > b.se {
            return Err(CORRUPT_DATA);
        }
        p.set(blk, k, clamp16(extend(br.read_bits(s)?, s).wrapping_shl(b.al)));
        k += 1;
    }
    Ok(())
}
