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

use super::ac_first::{ac_first, Band};
use super::ac_refine::ac_refine;
use super::blocks::seq_block;
use super::dc_scans::{dc_first, dc_refine};
use super::huff::Coder;
use super::plane::Plane;
use super::scan::Scan;
use crate::image::jpeg::bits::BitReader;
use crate::image::types::DecodeError;

/// One block of scan component `i`: a whole sequential block, or the
/// DC or AC band a progressive scan codes, first pass or refinement.
pub(super) fn block(
    (progressive, s, i): (bool, &Scan, usize),
    (dc, ac): (&[Coder], &[Coder]),
    (pred, eobrun): (&mut i32, &mut u32),
    p: &mut Plane,
    blk: usize,
    br: &mut BitReader,
) -> Result<(), DecodeError> {
    let (dc, ac) = (&dc[s.td[i]], &ac[s.ta[i]]);
    let band = Band { ss: s.ss, se: s.se, al: s.al };
    if !progressive {
        return seq_block(br, (dc, ac), pred, p, blk);
    }
    match (s.ss == 0, s.ah > 0) {
        (true, false) => dc_first(br, dc, pred, s.al, p, blk),
        (true, true) => dc_refine(br, s.al, p, blk),
        (false, false) => ac_first(br, ac, band, eobrun, p, blk),
        (false, true) => ac_refine(br, ac, band, eobrun, p, blk),
    }
}
