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

//! A written table to damage, and the CRCs made good again after, so a
//! test changes one thing at a time. Mounted beside `kernel_gpt` and
//! `written`.

use crate::kernel_gpt::crc32::crc32;
use crate::written::written;
use nonos_disk::MIN_DISK_SECTORS;

pub const T: u64 = MIN_DISK_SECTORS;

/// The written primary header and its 128-entry array.
pub fn table() -> (Vec<u8>, Vec<u8>) {
    let (disk, _, _) = written(T);
    (disk.read_sectors(1, 1), disk.read_sectors(2, 32))
}

/// Both CRCs made good over what the header and array now hold.
pub fn seal(h: &mut [u8], array: &[u8]) {
    h[88..92].copy_from_slice(&crc32(array).to_le_bytes());
    let size = u32::from_le_bytes(h[12..16].try_into().unwrap()) as usize;
    h[16..20].fill(0);
    let crc = crc32(&h[..size.clamp(92, 512)]);
    h[16..20].copy_from_slice(&crc.to_le_bytes());
}

pub fn put(h: &mut [u8], at: usize, v: u64, width: usize) {
    h[at..at + width].copy_from_slice(&v.to_le_bytes()[..width]);
}
