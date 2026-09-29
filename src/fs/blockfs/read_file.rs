// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::file_consts::{INDEX_MAGIC, INDEX_MAGIC_FLAT, INDEX_PTR_BASE, MAX_PTRS, PTR_BYTES};
use super::read_file_flat::read_flat;
use super::read_u64::read_u64;
use super::tree_range::read_range;
use super::tree_reader::TreeReader;
use super::tree_sealed::{fault, SealedSource};
use super::{BlockFsError, BlockFsNode};
use crate::fs::cryptoblock::ReadAhead;

/// Read a file from its start into `out`; see `read_file_at`.
pub fn read_file(
    key: &[u8; 32],
    node: &BlockFsNode,
    out: &mut [u8],
) -> Result<usize, BlockFsError> {
    read_file_at(key, node, 0, out)
}

/// Read the file's bytes from `offset` into `out`, never past its size.
/// Returns how many bytes were copied. A file of any size can be read this
/// way with a buffer of any size.
pub fn read_file_at(
    key: &[u8; 32],
    node: &BlockFsNode,
    offset: u64,
    out: &mut [u8],
) -> Result<usize, BlockFsError> {
    if node.first_record_lba == 0 || node.size == 0 {
        return Ok(0);
    }
    let index = crate::fs::cryptoblock::read(key, node.first_record_lba)
        .map_err(BlockFsError::CryptoBlock)?;
    if index[0..8] == INDEX_MAGIC_FLAT[..] {
        return read_flat(key, node, &index, offset, out);
    }
    if index[0..8] != INDEX_MAGIC[..] {
        return Err(BlockFsError::InvalidRecord);
    }
    let mut root = [0u64; MAX_PTRS];
    for (i, slot) in root.iter_mut().enumerate() {
        *slot = read_u64(&index, INDEX_PTR_BASE + i * PTR_BYTES);
    }
    let mut source = SealedSource { key, ahead: ReadAhead::new() };
    let mut reader = TreeReader::new(root);
    read_range(&mut source, &mut reader, node.size, offset, out).map_err(fault)
}
