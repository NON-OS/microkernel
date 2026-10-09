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
use super::tree_store::{Block, BlockSource, BlockStore};
use super::{BlockFsError, BlockFsMount};
use crate::fs::cryptoblock::{ReadAhead, PLAIN_BLOCK_BYTES};

/// Reads only. Data blocks come a run at a time, into a run the read cache
/// may keep for the file's next read; pointer blocks come one by one unless
/// the run holds them, so the run is not thrown away for them.
pub(super) struct SealedSource<'a> {
    pub key: &'a [u8; 32],
    pub ahead: &'a mut ReadAhead,
}

/// Reads, writes and allocates, for a file being written; the commit after
/// the last write flushes them all.
pub(super) struct SealedStore<'a> {
    pub key: &'a [u8; 32],
    pub mount: &'a mut BlockFsMount,
}

impl BlockSource for SealedSource<'_> {
    type Error = BlockFsError;
    fn get(&mut self, lba: u64) -> Result<Block, BlockFsError> {
        let mut block = [0u8; PLAIN_BLOCK_BYTES];
        self.get_into(lba, &mut block)?;
        Ok(block)
    }
    fn get_into(&mut self, lba: u64, out: &mut Block) -> Result<(), BlockFsError> {
        self.ahead.read_into(self.key, lba, out).map_err(BlockFsError::CryptoBlock)
    }
    fn get_pointer(&mut self, lba: u64) -> Result<Block, BlockFsError> {
        if self.ahead.holds(lba) {
            return self.get(lba);
        }
        crate::fs::cryptoblock::read(self.key, lba).map_err(BlockFsError::CryptoBlock)
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
