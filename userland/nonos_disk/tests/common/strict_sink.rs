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

//! A sink that refuses what a block driver refuses, stores nothing, reads
//! back zeros, and records every write and when the flush came.

use nonos_disk::{BlockSink, SinkError, SECTOR_SIZE};

pub struct StrictSink {
    pub sectors: u64,
    pub writes: Vec<(u64, usize)>,
    pub flushed_after: Option<usize>,
}

impl BlockSink for StrictSink {
    fn capacity_sectors(&mut self) -> Result<u64, SinkError> {
        Ok(self.sectors)
    }
    fn write_at(&mut self, lba: u64, data: &[u8]) -> Result<(), SinkError> {
        self.writes.push((lba, data.len()));
        if data.is_empty() || !data.len().is_multiple_of(SECTOR_SIZE) {
            return Err(SinkError(-22));
        }
        if lba + (data.len() / SECTOR_SIZE) as u64 > self.sectors {
            return Err(SinkError(-6));
        }
        Ok(())
    }
    fn read_at(&mut self, _lba: u64, out: &mut [u8]) -> Result<(), SinkError> {
        out.fill(0);
        Ok(())
    }
    fn flush(&mut self) -> Result<(), SinkError> {
        self.flushed_after = Some(self.writes.len());
        Ok(())
    }
}
