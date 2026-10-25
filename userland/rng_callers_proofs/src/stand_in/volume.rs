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

//! The block filesystem under the data volume. Nothing ever mounts, so each
//! volume is formatted; each format keeps, per thread, its key and uuid.

use std::cell::RefCell;

thread_local! {
    static FORMATS: RefCell<Vec<([u8; 32], [u8; 16])>> = const { RefCell::new(Vec::new()) };
}

/// Each format on this thread: its key and its uuid.
pub fn formats() -> Vec<([u8; 32], [u8; 16])> {
    FORMATS.with_borrow(Clone::clone)
}

pub mod blockfs {
    use crate::fs::cryptoblock::CryptoBlockError;
    use crate::hardware::block_device::BlockDeviceError;

    pub const HEADER_RING_SECTORS: u64 = 256;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BlockFsError {
        BlockDevice(BlockDeviceError),
        CryptoBlock(CryptoBlockError),
        NotFound,
        NotFormatted,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BlockFsSuperblock {
        pub sectors: u64,
        pub free_lba: u64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BlockFsMount {
        pub header_lba: u64,
        pub superblock: BlockFsSuperblock,
    }

    #[derive(Default)]
    pub struct ReadCache;
    impl ReadCache {
        pub const fn new() -> Self {
            ReadCache
        }
    }

    pub fn mount(_key: &[u8; 32]) -> Result<BlockFsMount, BlockFsError> {
        Err(BlockFsError::NotFormatted)
    }

    pub fn format(key: &[u8; 32], uuid: [u8; 16]) -> Result<BlockFsMount, BlockFsError> {
        super::FORMATS.with_borrow_mut(|f| f.push((*key, uuid)));
        let superblock = BlockFsSuperblock { sectors: 512, free_lba: 1 };
        Ok(BlockFsMount { header_lba: 0, superblock })
    }
}
