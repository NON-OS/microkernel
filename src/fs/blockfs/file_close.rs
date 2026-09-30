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
use super::index_block::encode;
use super::tree_sealed::{fault, SealedStore};
use super::tree_store::BlockStore;
use super::tree_writer::TreeWriter;
use super::write_node::write_node;
use super::{BlockFsError, BlockFsMount, BlockFsNode};

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
    let index = encode(&tree.root, tree.data_blocks);
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
