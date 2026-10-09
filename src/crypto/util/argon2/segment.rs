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

//! Filling one segment of one lane, RFC 9106 sections 3.2 and 3.4.

use super::address::Addresses;
use super::block::{compress, Block};
use super::index::{Position, Shape};
use super::memory::Memory;

/// Fill the segment `at` names; `at.index` is ignored. `scratch` carries
/// the block being made and is left holding the last one.
pub(super) fn fill_segment(shape: &Shape, mem: &mut Memory, at: Position, scratch: &mut Block) {
    let independent = shape.independent(at);
    let mut addresses = Addresses::new(shape, at);
    /*
     * The first two blocks of each lane come from H0, so the first
     * segment starts at its third block, with its first address block
     * made up front.
     */
    let first = if at.pass == 0 && at.slice == 0 { 2 } else { 0 };
    if independent && first == 2 {
        addresses.next();
    }
    let lane_base = at.lane * shape.lane_len;
    for index in first..shape.seg_len {
        let column = at.slice * shape.seg_len + index;
        let cur = lane_base + column;
        let prev = if column == 0 { lane_base + shape.lane_len - 1 } else { cur - 1 };
        let rand = if independent {
            if index % 128 == 0 {
                addresses.next();
            }
            addresses.word(index)
        } else {
            mem.at(prev)[0]
        };
        let ref_lane = if at.pass == 0 && at.slice == 0 {
            at.lane
        } else {
            ((rand >> 32) as usize) % shape.lanes
        };
        let here = Position { index, ..at };
        let ref_col = shape.ref_index(here, rand & 0xFFFF_FFFF, ref_lane == at.lane);
        let reference = ref_lane * shape.lane_len + ref_col;
        let keep = at.pass > 0;
        if keep {
            *scratch = *mem.at(cur);
        }
        compress(mem.at(prev), mem.at(reference), scratch, keep);
        *mem.at_mut(cur) = *scratch;
    }
}
