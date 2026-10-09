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

use super::Frame;
use crate::image::types::DecodeError;

/// One frame component: its id, sampling factors, quant table, the
/// MCU-padded block grid (`bw` x `bh`) and its own grid (`ow` x `oh`), which
/// a scan of that component alone walks.
#[derive(Clone, Copy, Default)]
pub struct Comp {
    pub id: u8,
    pub h: usize,
    pub v: usize,
    pub tq: usize,
    pub bw: usize,
    pub bh: usize,
    pub ow: usize,
    pub oh: usize,
}

/// Size the MCU grid and each component's padded and own block grids;
/// every sampling factor must divide the largest, or the component grids
/// have no whole-block size and the frame is refused as BadDimensions.
pub(super) fn place(f: &mut Frame) -> Result<(), DecodeError> {
    f.mcux = f.w.div_ceil(8 * f.hmax);
    f.mcuy = f.h.div_ceil(8 * f.vmax);
    for c in f.comps.iter_mut().take(f.n) {
        if f.hmax % c.h != 0 || f.vmax % c.v != 0 {
            return Err(DecodeError::BadDimensions);
        }
        (c.bw, c.bh) = (f.mcux * c.h, f.mcuy * c.v);
        c.ow = (f.w * c.h).div_ceil(f.hmax).div_ceil(8);
        c.oh = (f.h * c.v).div_ceil(f.vmax).div_ceil(8);
    }
    Ok(())
}
