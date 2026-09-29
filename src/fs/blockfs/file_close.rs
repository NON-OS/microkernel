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

//! Ending a file write: the index block, the node, and the commit.

use super::commit::commit;
use super::file_consts::{INDEX_COUNT_OFFSET, INDEX_MAGIC, INDEX_PTR_BASE, PTR_BYTES};
use super::tree_sealed::{fault, SealedStore};
use super::tree_store::BlockStore;
use super::tree_writer::TreeWriter;
use super::write_node::write_node;
use super::write_u32::write_u32;
use super::write_u64::write_u64;
use super::{BlockFsError, BlockFsMount, BlockFsNode};
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

pub(super) fn close_file(
    key: &[u8; 32],
    mount: &mut BlockFsMount,
    node_lba: u64,
    node: &mut BlockFsNode,
    mut tree: TreeWriter,
    size: u64,
) -> Result<(), BlockFsError> {
    let mut store = SealedStore { key, mount };
    tree.finish(&mut store).map_err(fault)?;
    let index_lba = store.alloc()?;
    let mut index = [0u8; PLAIN_BLOCK_BYTES];
    index[0..8].copy_from_slice(&INDEX_MAGIC);
    /*
     * At most 13,179,714 data blocks: the count fits its u32.
     */
    write_u32(&mut index, INDEX_COUNT_OFFSET, tree.data_blocks as u32);
    for (i, lba) in tree.root.iter().enumerate() {
        write_u64(&mut index, INDEX_PTR_BASE + i * PTR_BYTES, *lba);
    }
    /*
     * The flush in this write covers every data and pointer block written
     * before it, so the index never reaches the disk ahead of its blocks.
     */
    crate::fs::cryptoblock::write(key, index_lba, &index).map_err(BlockFsError::CryptoBlock)?;
    node.first_record_lba = index_lba;
    node.size = size;
    node.blocks = 1 + tree.data_blocks + tree.pointer_blocks;
    commit(key, mount)?;
    write_node(key, node_lba, node)
}
