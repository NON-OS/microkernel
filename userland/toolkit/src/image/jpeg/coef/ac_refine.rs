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

use super::ac_correct::correct;
use super::ac_first::Band;
use super::huff::Coder;
use super::plane::Plane;
use crate::image::jpeg::bits::BitReader;
use crate::image::jpeg::coef::refuse::CORRUPT_DATA;
use crate::image::types::DecodeError;

/// An AC refinement scan (ITU T.81 G.1.2.3, as libjpeg decodes it): a nonzero
/// coefficient takes a correction bit that may grow it by 2^al; new ones
/// arrive as +-2^al after a run of still-zero positions.
pub fn ac_refine(
    br: &mut BitReader,
    ac: &Coder,
    b: Band,
    eobrun: &mut u32,
    p: &mut Plane,
    blk: usize,
) -> Result<(), DecodeError> {
    let p1 = 1i32 << b.al;
    let mut k = b.ss;
    if *eobrun == 0 {
        while k <= b.se {
            let rs = ac.symbol(br)?;
            let (mut r, s) = ((rs >> 4) as i32, rs & 15);
            if s == 0 && r != 15 {
                *eobrun = (1 << r) + br.read_bits(r as u32)?;
                break;
            }
            let fresh = if s == 0 { 0 } else { [-p1, p1][br.read_bits(1)? as usize & 1] };
            while k <= b.se {
                if p.nonzero(blk, k) {
                    correct(br, p, blk, k, p1)?;
                } else {
                    r -= 1;
                    if r < 0 {
                        break;
                    }
                }
                k += 1;
            }
            if fresh != 0 {
                (k <= b.se).then_some(()).ok_or(CORRUPT_DATA)?;
                p.set(blk, k, fresh as i16);
            }
            k += 1;
        }
    }
    if *eobrun > 0 {
        for k in k..=b.se {
            if p.nonzero(blk, k) {
                correct(br, p, blk, k, p1)?;
            }
        }
        *eobrun -= 1;
    }
    Ok(())
}
