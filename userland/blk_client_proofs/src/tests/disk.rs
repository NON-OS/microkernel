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

//! A disk of 4096-byte blocks in memory, checking every request as the
//! NVMe driver does: a whole number of its blocks, no more than one request
//! moves, inside the disk.

use crate::chunks::Native;
use crate::geometry::Geometry;

pub const BLOCK: usize = 4096;

pub struct Disk {
    pub bytes: Vec<u8>,
    /// Each request as (write, block, blocks).
    pub sent: Vec<(bool, u64, usize)>,
    /// A block a request touching fails, as a driver's E_IO.
    pub bad: Option<u64>,
}

impl Disk {
    pub fn new(blocks: usize) -> Disk {
        let bytes = (0..blocks * BLOCK).map(|i| (i / 512) as u8).collect();
        Disk { bytes, sent: Vec::new(), bad: None }
    }

    fn at(&mut self, write: bool, lba: u64, len: usize) -> Result<usize, i32> {
        assert!(len > 0 && len.is_multiple_of(BLOCK) && len <= 32 * 1024, "{len}-byte request");
        let start = lba as usize * BLOCK;
        assert!(start + len <= self.bytes.len(), "past the end of the disk");
        self.sent.push((write, lba, len / BLOCK));
        let hit = self.bad.is_some_and(|b| (lba..lba + (len / BLOCK) as u64).contains(&b));
        if hit {
            Err(-5)
        } else {
            Ok(start)
        }
    }
}

impl Native for Disk {
    type Error = i32;
    fn read_native(&mut self, lba: u64, out: &mut [u8]) -> Result<(), i32> {
        let at = self.at(false, lba, out.len())?;
        out.copy_from_slice(&self.bytes[at..at + out.len()]);
        Ok(())
    }
    fn write_native(&mut self, lba: u64, data: &[u8]) -> Result<(), i32> {
        let at = self.at(true, lba, data.len())?;
        self.bytes[at..at + data.len()].copy_from_slice(data);
        Ok(())
    }
}

pub fn g() -> Geometry {
    Geometry::nvme(4096, 0).expect("4Kn")
}
