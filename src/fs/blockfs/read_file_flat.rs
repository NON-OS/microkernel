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

//! Files written before indirection: one index block of up to 58 data
//! pointers. They are read as they were written and never written again in
//! this layout.

use super::file_consts::{DATA_BYTES, INDEX_COUNT_OFFSET, INDEX_PTR_BASE, MAX_PTRS, PTR_BYTES};
use super::read_u32::read_u32;
use super::read_u64::read_u64;
use super::tree_store::Block;
use super::{BlockFsError, BlockFsNode};

pub(super) fn read_flat(
    key: &[u8; 32],
    node: &BlockFsNode,
    index: &Block,
    offset: u64,
    out: &mut [u8],
) -> Result<usize, BlockFsError> {
    let count = (read_u32(index, INDEX_COUNT_OFFSET) as usize).min(MAX_PTRS);
    let size = node.size.min((count * DATA_BYTES) as u64);
    if offset >= size {
        return Ok(0);
    }
    let end = size.min(offset.saturating_add(out.len() as u64));
    let mut pos = offset;
    while pos < end {
        let i = (pos / DATA_BYTES as u64) as usize;
        let within = (pos % DATA_BYTES as u64) as usize;
        let lba = read_u64(index, INDEX_PTR_BASE + i * PTR_BYTES);
        let block = crate::fs::cryptoblock::read(key, lba).map_err(BlockFsError::CryptoBlock)?;
        let take = ((DATA_BYTES - within) as u64).min(end - pos) as usize;
        let at = (pos - offset) as usize;
        out[at..at + take].copy_from_slice(&block[within..within + take]);
        pos += take as u64;
    }
    Ok((end - offset) as usize)
}
