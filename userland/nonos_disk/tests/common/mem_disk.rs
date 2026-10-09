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

//! A disk in memory that keeps only the sectors written to it, so a test
//! can install onto a disk of any size. It refuses what a block driver
//! refuses: a transfer that is empty, not whole sectors, or past the end.

use std::collections::BTreeMap;

use nonos_disk::{BlockSink, SinkError, SECTOR_SIZE};

pub struct MemDisk {
    pub sectors: u64,
    pub written: BTreeMap<u64, [u8; SECTOR_SIZE]>,
}

impl MemDisk {
    pub fn new(sectors: u64) -> MemDisk {
        MemDisk { sectors, written: BTreeMap::new() }
    }

    /// `n` sectors from `lba`, zero where nothing was written.
    pub fn read_sectors(&self, lba: u64, n: usize) -> Vec<u8> {
        let mut out = vec![0u8; n * SECTOR_SIZE];
        for (i, chunk) in out.chunks_mut(SECTOR_SIZE).enumerate() {
            if let Some(s) = self.written.get(&(lba + i as u64)) {
                chunk.copy_from_slice(s);
            }
        }
        out
    }

    fn check(&self, lba: u64, len: usize) -> Result<(), SinkError> {
        if len == 0 || !len.is_multiple_of(SECTOR_SIZE) {
            return Err(SinkError(-22));
        }
        let end = lba.checked_add((len / SECTOR_SIZE) as u64);
        end.filter(|&e| e <= self.sectors).map(|_| ()).ok_or(SinkError(-6))
    }
}

impl BlockSink for MemDisk {
    fn capacity_sectors(&mut self) -> Result<u64, SinkError> {
        Ok(self.sectors)
    }

    fn write_at(&mut self, lba: u64, data: &[u8]) -> Result<(), SinkError> {
        self.check(lba, data.len())?;
        for (i, chunk) in data.chunks(SECTOR_SIZE).enumerate() {
            self.written.insert(lba + i as u64, chunk.try_into().unwrap());
        }
        Ok(())
    }
    fn read_at(&mut self, lba: u64, out: &mut [u8]) -> Result<(), SinkError> {
        self.check(lba, out.len())?;
        out.copy_from_slice(&self.read_sectors(lba, out.len() / SECTOR_SIZE));
        Ok(())
    }
    fn flush(&mut self) -> Result<(), SinkError> {
        Ok(())
    }
}
