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

//! Memory the modelled host reaches by bus address. In the model a bus
//! address is the host address, so the device reads and writes the regions
//! the driver hands it directly.

use crate::ufs::env::DmaBuf;

/// A region, 8-byte aligned, kept alive while the model can reach it.
pub struct Region {
    words: Vec<u64>,
}

impl Region {
    pub fn new(bytes: usize) -> Self {
        Self { words: vec![0; bytes.div_ceil(8)] }
    }

    pub fn buf(&self) -> DmaBuf {
        let va = self.words.as_ptr() as u64;
        DmaBuf { va, bus: va, len: self.words.len() * 8 }
    }
}

/// # Safety
/// `bus` must be inside a live `Region` for `out.len()` bytes.
pub unsafe fn read(bus: u64, out: &mut [u8]) {
    std::ptr::copy_nonoverlapping(bus as *const u8, out.as_mut_ptr(), out.len());
}

/// # Safety
/// As `read`, for `src.len()` bytes.
pub unsafe fn write(bus: u64, src: &[u8]) {
    std::ptr::copy_nonoverlapping(src.as_ptr(), bus as *mut u8, src.len());
}

pub fn be32(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

pub fn le32(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}
