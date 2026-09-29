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

//! The index trees on the real disk: every block sealed to its own LBA.

use super::alloc_block::alloc_block;
use super::tree_store::{Block, BlockSource, BlockStore, TreeFault};
use super::{BlockFsError, BlockFsMount};
use crate::fs::cryptoblock::ReadAhead;

/// Reads only: a file being read allocates and writes nothing. Its blocks
/// are fetched a run at a time; the run lives only as long as this source,
/// one read under the volume's lock, so no write can make it stale.
pub(super) struct SealedSource<'a> {
    pub key: &'a [u8; 32],
    pub ahead: ReadAhead,
}

/// Reads, writes and allocates, for a file being written. Writes are not
/// flushed one by one; the commit that follows the last of them flushes.
pub(super) struct SealedStore<'a> {
    pub key: &'a [u8; 32],
    pub mount: &'a mut BlockFsMount,
}

impl BlockSource for SealedSource<'_> {
    type Error = BlockFsError;
    fn get(&mut self, lba: u64) -> Result<Block, BlockFsError> {
        self.ahead.read(self.key, lba).map_err(BlockFsError::CryptoBlock)
    }
}

impl BlockSource for SealedStore<'_> {
    type Error = BlockFsError;
    fn get(&mut self, lba: u64) -> Result<Block, BlockFsError> {
        crate::fs::cryptoblock::read(self.key, lba).map_err(BlockFsError::CryptoBlock)
    }
}

impl BlockStore for SealedStore<'_> {
    fn alloc(&mut self) -> Result<u64, BlockFsError> {
        alloc_block(self.mount)
    }
    fn put(&mut self, lba: u64, block: &Block) -> Result<(), BlockFsError> {
        crate::fs::cryptoblock::write_deferred(self.key, lba, block)
            .map_err(BlockFsError::CryptoBlock)
    }
}

/// A tree fault in the filesystem's terms.
pub(super) fn fault(f: TreeFault<BlockFsError>) -> BlockFsError {
    match f {
        TreeFault::Store(e) => e,
        TreeFault::TooLarge => BlockFsError::OutOfSpace,
        TreeFault::Hole => BlockFsError::InvalidRecord,
    }
}
