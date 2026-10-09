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

//! The kernel's sealed source, modelled on the counting disk. Data blocks
//! come through a run of RUN sectors fetched from the first one missed, as
//! cryptoblock's ReadAhead fetches them; pointer blocks come one sector at
//! a time unless the run holds them, as SealedSource reads them. A run
//! holds copies, as the kernel's holds the sealed sectors it fetched, so a
//! block written after its run was fetched is not in it.

use super::disk::Disk;
use super::tree_store::{Block, BlockSource};

/// Sectors in one run, as cryptoblock's RUN_SECTORS.
pub const RUN: u64 = 64;

/// Copies of the sectors fetched last, from `start`.
#[derive(Default)]
pub struct Run {
    start: u64,
    blocks: Vec<Block>,
}

impl Run {
    fn holds(&self, lba: u64) -> bool {
        lba >= self.start && lba - self.start < self.blocks.len() as u64
    }
}

pub struct Source<'a> {
    pub disk: &'a mut Disk,
    pub run: &'a mut Run,
}

impl BlockSource for Source<'_> {
    type Error = &'static str;
    fn get(&mut self, lba: u64) -> Result<Block, &'static str> {
        if !self.run.holds(lba) {
            if lba == 0 || lba > self.disk.last {
                return Err("past the disk");
            }
            let count = RUN.min(self.disk.last + 1 - lba);
            self.run.blocks = (lba..lba + count).map(|l| self.disk.now(l)).collect();
            self.run.start = lba;
            self.disk.fetched += count;
            self.disk.requests += 1;
        }
        Ok(self.run.blocks[(lba - self.run.start) as usize])
    }
    fn get_pointer(&mut self, lba: u64) -> Result<Block, &'static str> {
        if self.run.holds(lba) {
            return self.get(lba);
        }
        self.disk.get(lba)
    }
}
