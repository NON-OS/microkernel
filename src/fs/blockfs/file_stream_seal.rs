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

//! Sealing a streamed file's blocks, and closing it.

use super::file_close::close_file;
use super::file_stream::FileStream;
use super::tree_store::BlockStore;
use super::{error::fault, tree_sealed::SealedStore};
use super::{BlockFsError, BlockFsMount, BlockFsNode};
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

impl FileStream {
    /// Seal the last partial block, write the index, and point the node at it.
    pub fn finish(
        mut self,
        key: &[u8; 32],
        mount: &mut BlockFsMount,
        node_lba: u64,
        node: &mut BlockFsNode,
    ) -> Result<(), BlockFsError> {
        if self.tail_len > 0 {
            self.seal_tail(key, mount)?;
        }
        close_file(key, mount, node_lba, node, self.tree, self.size)
    }

    pub(super) fn seal_tail(
        &mut self,
        key: &[u8; 32],
        mount: &mut BlockFsMount,
    ) -> Result<(), BlockFsError> {
        let mut store = SealedStore { key, mount };
        let lba = store.alloc()?;
        store.put(lba, &self.tail)?;
        self.tree.push(&mut store, lba).map_err(fault)?;
        self.tail = [0u8; PLAIN_BLOCK_BYTES];
        self.tail_len = 0;
        Ok(())
    }
}
