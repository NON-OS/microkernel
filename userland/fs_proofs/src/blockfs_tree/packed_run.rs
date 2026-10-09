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

//! A pointer block as the packed store keeps it: an even run of LBAs, then
//! zeros, as its first LBA, step and count.

use super::file_consts::FANOUT;
use super::tree_ptrs::{encode, entry};
use super::tree_store::Block;

pub type Run = (u64, u64, u8);

/// The block a run stands for.
pub fn unpack(&(first, step, count): &Run) -> Block {
    let mut ptrs = [0u64; FANOUT];
    for (i, p) in ptrs[..count as usize].iter_mut().enumerate() {
        *p = first.wrapping_add(step.wrapping_mul(i as u64));
    }
    encode(&ptrs)
}

/// The run read off a block's first two entries and its nonzero count. A
/// block that is not such a run does not unpack to itself; the store keeps
/// that one whole.
pub fn pack(b: &Block) -> Run {
    let count = (0..FANOUT).take_while(|&i| entry(b, i) != 0).count() as u8;
    (entry(b, 0), entry(b, 1).wrapping_sub(entry(b, 0)), count)
}
