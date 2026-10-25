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

//! What the reader is handed of a disk: sector 0, the primary header (0)
//! at LBA 1 and the backup (1) at the last LBA, and each one's array where
//! it was built. `seal` makes a header's CRCs good again, so a lie told in
//! a field reaches the reader with every CRC right.

use super::crc::crc32;

/// Every array byte a header could name, and an entry past 128 KiB.
pub const ARRAY_ROOM: usize = 128 * 1024 + 2048;

#[derive(Clone)]
pub struct Disk {
    pub sectors: u64,
    pub mbr: Vec<u8>,
    pub heads: [Vec<u8>; 2],
    pub arrays: [Vec<u8>; 2],
    /// Where each array was built; a header naming another LBA reads zeros.
    pub array_lba: [u64; 2],
}

impl Disk {
    pub fn get(&self, w: usize, at: usize, width: usize) -> u64 {
        let mut b = [0u8; 8];
        b[..width].copy_from_slice(&self.heads[w][at..at + width]);
        u64::from_le_bytes(b)
    }

    pub fn put(&mut self, w: usize, at: usize, v: u64, width: usize) {
        self.heads[w][at..at + width].copy_from_slice(&v.to_le_bytes()[..width]);
    }

    /// Entry `i` of header `w`'s array, as its count and size place it.
    pub fn entry(&mut self, w: usize, i: usize) -> &mut [u8] {
        let size = self.get(w, 84, 4) as usize;
        &mut self.arrays[w][i * size..(i + 1) * size]
    }

    /// The bytes at `lba` a header's array read gets, `len` of them.
    pub fn read(&self, lba: u64, len: usize) -> Vec<u8> {
        match self.array_lba.iter().position(|&at| at == lba) {
            Some(w) => self.arrays[w][..len.min(ARRAY_ROOM)].to_vec(),
            None => vec![0; len],
        }
    }

    /// Header `w`'s array CRC over the entries it names, then its own.
    pub fn seal(&mut self, w: usize) {
        let bytes = (self.get(w, 80, 4) * self.get(w, 84, 4)).min(ARRAY_ROOM as u64) as usize;
        self.put(w, 88, u64::from(crc32(&self.arrays[w][..bytes])), 4);
        self.reseal_header(w);
    }

    pub fn reseal_header(&mut self, w: usize) {
        let size = (self.get(w, 12, 4) as usize).min(512);
        self.put(w, 16, 0, 4);
        let crc = crc32(&self.heads[w][..size]);
        self.put(w, 16, u64::from(crc), 4);
    }
}
