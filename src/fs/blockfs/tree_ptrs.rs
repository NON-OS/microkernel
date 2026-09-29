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

//! A pointer block as bytes: FANOUT little-endian LBAs, the rest zero.

use super::file_consts::{FANOUT, PTR_BYTES};
use super::read_u64::read_u64;
use super::tree_store::Block;
use super::write_u64::write_u64;
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

pub(crate) fn encode(ptrs: &[u64; FANOUT]) -> Block {
    let mut block = [0u8; PLAIN_BLOCK_BYTES];
    for (i, lba) in ptrs.iter().enumerate() {
        write_u64(&mut block, i * PTR_BYTES, *lba);
    }
    block
}

pub(crate) fn entry(block: &Block, i: usize) -> u64 {
    read_u64(block, i * PTR_BYTES)
}
