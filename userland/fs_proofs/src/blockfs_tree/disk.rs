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

//! A disk that counts every sector fetched from it. A data block never
//! written holds bytes made from its LBA, so a file of many gigabytes needs
//! memory only for its pointer blocks and whatever is written over.

use std::collections::HashMap;

use super::numbered_file::numbered;
use super::tree_store::{Block, BlockSource, BlockStore};

#[derive(Default)]
pub struct Disk {
    /// The last LBA allocated; LBA 0 is never one.
    pub last: u64,
    kept: HashMap<u64, Block>,
    /// Sectors fetched, and the device requests that fetched them.
    pub fetched: u64,
    pub requests: u64,
    /// Writes made, counted as cryptoblock's epoch counts them.
    pub epoch: u64,
}

impl Disk {
    /// The block at `lba` as it is now, fetching nothing.
    pub fn now(&self, lba: u64) -> Block {
        self.kept.get(&lba).copied().unwrap_or_else(|| numbered(lba))
    }
}

impl BlockSource for Disk {
    type Error = &'static str;
    fn get(&mut self, lba: u64) -> Result<Block, &'static str> {
        if lba == 0 || lba > self.last {
            return Err("past the disk");
        }
        self.fetched += 1;
        self.requests += 1;
        Ok(self.now(lba))
    }
}

impl BlockStore for Disk {
    fn alloc(&mut self) -> Result<u64, &'static str> {
        self.last += 1;
        Ok(self.last)
    }
    fn put(&mut self, lba: u64, block: &Block) -> Result<(), &'static str> {
        self.kept.insert(lba, *block);
        self.epoch += 1;
        Ok(())
    }
}
