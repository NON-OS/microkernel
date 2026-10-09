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

use super::frame::Frame;
use super::huff::Coder;
use super::plane::Plane;
use super::scan::Scan;
use super::walk_block::block;
use crate::image::jpeg::bits::BitReader;
use crate::image::types::DecodeError;

/// Decode one scan's entropy-coded data into the coefficient planes. A
/// scan of one component walks its own block grid, an interleaved one the
/// MCUs; every `ri` units a restart marker resets the predictions. Data
/// that ends early stops the scan with what it decoded so far.
pub fn walk(
    f: &Frame,
    s: &Scan,
    planes: &mut [Plane],
    coders: (&[Coder], &[Coder]),
    ri: usize,
    br: &mut BitReader,
) -> Result<(), DecodeError> {
    let one = &f.comps[s.comp[0]];
    let units = if s.n == 1 { one.ow * one.oh } else { f.mcux * f.mcuy };
    let (mut preds, mut eobrun) = ([0i32; 4], 0u32);
    for u in 0..units {
        if ri > 0 && u > 0 && u % ri == 0 {
            if !super::marker::restart(br) {
                return Ok(());
            }
            (preds, eobrun) = ([0; 4], 0);
        }
        for i in 0..s.n {
            let c = &f.comps[s.comp[i]];
            let (ch, cv) = if s.n == 1 { (1, 1) } else { (c.h, c.v) };
            for v in 0..cv {
                for h in 0..ch {
                    let blk = if s.n == 1 {
                        (u / c.ow) * c.bw + u % c.ow
                    } else {
                        ((u / f.mcux) * c.v + v) * c.bw + (u % f.mcux) * c.h + h
                    };
                    let st = (&mut preds[i], &mut eobrun);
                    let p = &mut planes[s.comp[i]];
                    match block((f.progressive, s, i), coders, st, p, blk, br) {
                        Err(DecodeError::Truncated) => return Ok(()),
                        r => r?,
                    }
                }
            }
        }
    }
    Ok(())
}
