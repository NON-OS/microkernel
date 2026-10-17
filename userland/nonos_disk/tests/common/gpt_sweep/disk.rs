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

//! What the reader is handed of a disk: sector 0, the primary header's
//! sector, and the bytes from the entry array's LBA on. The header and
//! array are kept as built; `seal` makes their CRCs good again, so a lie
//! told in a field reaches the reader with every CRC right.

use super::crc::crc32;

/// The most array bytes any header could have read, and a sector over.
pub const ARRAY_ROOM: usize = 128 * 1024 + 512;

#[derive(Clone)]
pub struct Disk {
    pub sectors: u64,
    pub mbr: Vec<u8>,
    pub header: Vec<u8>,
    pub array: Vec<u8>,
}

impl Disk {
    pub fn get(&self, at: usize, width: usize) -> u64 {
        let mut b = [0u8; 8];
        b[..width].copy_from_slice(&self.header[at..at + width]);
        u64::from_le_bytes(b)
    }

    pub fn put(&mut self, at: usize, v: u64, width: usize) {
        self.header[at..at + width].copy_from_slice(&v.to_le_bytes()[..width]);
    }

    /// Entry `i` as the header's count and size place it.
    pub fn entry(&mut self, i: usize) -> &mut [u8] {
        let size = self.get(84, 4) as usize;
        &mut self.array[i * size..(i + 1) * size]
    }

    /// The array CRC over the entries the header names, then the header's
    /// own over its HeaderSize bytes (held to the sector) with that field 0.
    pub fn seal(&mut self) {
        let bytes = (self.get(80, 4) * self.get(84, 4)).min(ARRAY_ROOM as u64) as usize;
        self.put(88, u64::from(crc32(&self.array[..bytes])), 4);
        self.reseal_header();
    }

    pub fn reseal_header(&mut self) {
        let size = (self.get(12, 4) as usize).min(512);
        self.put(16, 0, 4);
        let crc = crc32(&self.header[..size]);
        self.put(16, u64::from(crc), 4);
    }
}
