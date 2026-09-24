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

//! A block driver takes whole sectors and nothing else. The sink here
//! refuses what a driver refuses and stores nothing, so the image can be the
//! size of the real one: a loader that is an exact multiple of the sector
//! and a kernel that is not, on an eight gigabyte disk.

mod common;

use nonos_disk::{install, BlockSink, NonosImage, SinkError, SECTOR_SIZE};

struct StrictSink {
    sectors: u64,
    writes: Vec<(u64, usize)>,
}

impl BlockSink for StrictSink {
    fn capacity_sectors(&mut self) -> Result<u64, SinkError> {
        Ok(self.sectors)
    }
    fn write_at(&mut self, lba: u64, data: &[u8]) -> Result<(), SinkError> {
        self.writes.push((lba, data.len()));
        if data.is_empty() || data.len() % SECTOR_SIZE != 0 {
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
        Ok(())
    }
}

#[test]
fn every_write_is_whole_sectors() {
    // The loader fills its clusters exactly (a multiple of 4096), the kernel
    // does not: both shapes have been shipped, and the first one once queued
    // an empty tail write that the driver refused.
    let boot_efi = vec![0xAAu8; 14_225_408];
    let kernel_bin = vec![0x55u8; 89_971_099];
    let image = NonosImage { boot_efi: &boot_efi, kernel_bin: &kernel_bin, boot_cfg: b"x=1\n" };
    let mut sink = StrictSink { sectors: (8u64 << 30) / SECTOR_SIZE as u64, writes: Vec::new() };
    let result = install(&mut sink, &image, common::ENTROPY, &mut |_| {});
    let bad: Vec<_> = sink.writes.iter().filter(|(_, n)| *n == 0 || n % SECTOR_SIZE != 0).collect();
    assert!(bad.is_empty(), "writes a driver refuses: {bad:?}");
    let written: usize = sink.writes.iter().map(|(_, n)| n).sum();
    // The read-back sees zeros, so the install reports a mismatch; what
    // matters here is that the write phase went to the end.
    assert!(written > boot_efi.len() + kernel_bin.len(), "stopped early: {result:?}");
}
