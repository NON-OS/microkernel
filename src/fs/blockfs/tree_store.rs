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

//! What the index trees need from a disk: a fresh block, and a block written
//! or read by address. The kernel seals each one; the host proofs keep them
//! in memory, so the tree logic is tested exactly as it runs.

use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

pub(crate) type Block = [u8; PLAIN_BLOCK_BYTES];

/// Blocks read by address.
pub(crate) trait BlockSource {
    type Error;
    fn get(&mut self, lba: u64) -> Result<Block, Self::Error>;
    /// A pointer block. The sealed disk reads one on its own, so that the
    /// run of data blocks it fetched ahead is not thrown away for it.
    fn get_pointer(&mut self, lba: u64) -> Result<Block, Self::Error> {
        self.get(lba)
    }
}

/// Blocks allocated and written, as well as read.
pub(crate) trait BlockStore: BlockSource {
    fn alloc(&mut self) -> Result<u64, Self::Error>;
    fn put(&mut self, lba: u64, block: &Block) -> Result<(), Self::Error>;
}

/// Why a tree could not be written or read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TreeFault<E> {
    /// The disk refused.
    Store(E),
    /// More blocks than the index can describe.
    TooLarge,
    /// A pointer the file's size says must exist is zero.
    Hole,
}
