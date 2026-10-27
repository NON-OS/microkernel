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

//! A disk in memory that records every transfer as `(lba, sectors)`, so a
//! test can hold each write an update makes to the sectors it may touch,
//! and each read the read-back makes to the sectors that were written.

use nonos_disk::{BlockSink, SinkError, SECTOR_SIZE};

use crate::mem_disk::MemDisk;

pub struct Recorder {
    pub disk: MemDisk,
    pub writes: Vec<(u64, u64)>,
    pub reads: Vec<(u64, u64)>,
}

impl Recorder {
    pub fn new(disk: MemDisk) -> Recorder {
        Recorder { disk, writes: Vec::new(), reads: Vec::new() }
    }
}

impl BlockSink for Recorder {
    fn capacity_sectors(&mut self) -> Result<u64, SinkError> {
        self.disk.capacity_sectors()
    }
    fn write_at(&mut self, lba: u64, data: &[u8]) -> Result<(), SinkError> {
        self.writes.push((lba, (data.len() / SECTOR_SIZE) as u64));
        self.disk.write_at(lba, data)
    }
    fn read_at(&mut self, lba: u64, out: &mut [u8]) -> Result<(), SinkError> {
        self.reads.push((lba, (out.len() / SECTOR_SIZE) as u64));
        self.disk.read_at(lba, out)
    }
    fn flush(&mut self) -> Result<(), SinkError> {
        self.disk.flush()
    }
}

/// Every sector the transfers in `runs` cover.
pub fn sectors(runs: &[(u64, u64)]) -> std::collections::BTreeSet<u64> {
    runs.iter().flat_map(|&(lba, n)| lba..lba + n).collect()
}
