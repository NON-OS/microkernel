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

//! Every pass over the memory, slice by slice, each lane's segment in turn.

use super::block::Block;
use super::index::{Position, Shape, SLICES};
use super::memory::Memory;
use super::segment::fill_segment;
use super::wipe::wipe;

/*
 * Within a slice no lane references the segment another lane is filling,
 * so filling the lanes one after another gives what parallel lanes
 * give.
 */
pub(super) fn fill_all(shape: &Shape, mem: &mut Memory, between: &mut dyn FnMut()) {
    let mut scratch: Block = [0u64; 128];
    for pass in 0..shape.passes {
        for slice in 0..SLICES {
            for lane in 0..shape.lanes {
                let at = Position { pass, lane, slice, index: 0 };
                fill_segment(shape, mem, at, &mut scratch);
                between();
            }
        }
    }
    wipe(&mut scratch);
}
