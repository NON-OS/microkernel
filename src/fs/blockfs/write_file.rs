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

use super::file_close::close_file;
use super::file_consts::{DATA_BYTES, MAX_FILE_BYTES};
use super::tree_store::BlockStore;
use super::tree_writer::TreeWriter;
use super::{error::fault, tree_sealed::SealedStore};
use super::{BlockFsError, BlockFsMount, BlockFsNode};
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

/// Replace a file's contents with `data`, whole.
pub fn write_file(
    key: &[u8; 32],
    mount: &mut BlockFsMount,
    node_lba: u64,
    node: &mut BlockFsNode,
    data: &[u8],
) -> Result<(), BlockFsError> {
    if data.len() as u64 > MAX_FILE_BYTES {
        return Err(BlockFsError::OutOfSpace);
    }
    let mut tree = TreeWriter::new();
    let mut store = SealedStore { key, mount };
    for chunk in data.chunks(DATA_BYTES) {
        let mut block = [0u8; PLAIN_BLOCK_BYTES];
        block[..chunk.len()].copy_from_slice(chunk);
        let lba = store.alloc()?;
        store.put(lba, &block)?;
        tree.push(&mut store, lba).map_err(fault)?;
    }
    close_file(key, mount, node_lba, node, tree, data.len() as u64)
}
