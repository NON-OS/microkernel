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

//! A disk in memory and an image with recognisable bytes, shared by the
//! host tests. The disk records the flush so a test can assert on it.

use nonos_disk::{BlockSink, NonosImage, SinkError, SECTOR_SIZE};

pub struct MemDisk {
    pub bytes: Vec<u8>,
    pub flushed: bool,
}

impl MemDisk {
    pub fn new(mib: usize) -> MemDisk {
        MemDisk { bytes: vec![0u8; mib << 20], flushed: false }
    }
}

impl BlockSink for MemDisk {
    fn capacity_sectors(&mut self) -> Result<u64, SinkError> {
        Ok((self.bytes.len() / SECTOR_SIZE) as u64)
    }
    // The same refusals as a block driver: whole sectors, never none.
    fn write_at(&mut self, lba: u64, data: &[u8]) -> Result<(), SinkError> {
        if data.is_empty() || data.len() % SECTOR_SIZE != 0 {
            return Err(SinkError(-22));
        }
        let o = lba as usize * SECTOR_SIZE;
        self.bytes[o..o + data.len()].copy_from_slice(data);
        Ok(())
    }
    fn read_at(&mut self, lba: u64, out: &mut [u8]) -> Result<(), SinkError> {
        if out.is_empty() || out.len() % SECTOR_SIZE != 0 {
            return Err(SinkError(-22));
        }
        let o = lba as usize * SECTOR_SIZE;
        out.copy_from_slice(&self.bytes[o..o + out.len()]);
        Ok(())
    }
    fn flush(&mut self) -> Result<(), SinkError> {
        self.flushed = true;
        Ok(())
    }
}

/// Distinct content per file, shaped like the real image, scaled down.
pub struct Files {
    pub boot_efi: Vec<u8>,
    pub kernel_bin: Vec<u8>,
    pub boot_cfg: Vec<u8>,
}

pub fn files() -> Files {
    let pattern = |len: usize, seed: u32| -> Vec<u8> {
        (0..len).map(|i| (i as u32).wrapping_mul(2654435761).wrapping_add(seed) as u8).collect()
    };
    Files {
        boot_efi: pattern(1_400_000 + 7, 1),
        kernel_bin: pattern(8_500_000 + 333, 2),
        boot_cfg: b"timeout=0\ndefault=nonos\n".to_vec(),
    }
}

pub fn image(f: &Files) -> NonosImage<'_> {
    NonosImage { boot_efi: &f.boot_efi, kernel_bin: &f.kernel_bin, boot_cfg: &f.boot_cfg }
}

pub const ENTROPY: [u8; 36] = [0x5A; 36];
