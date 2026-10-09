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

/*
 * Whole sectors off a disk, or nothing: every question this module asks a
 * disk has the answer "no" when the disk will not say.
 */

use alloc::vec::Vec;

use crate::sink::{BlockSink, SECTOR_SIZE};

pub fn sectors(disk: &mut dyn BlockSink, lba: u64, count: u64) -> Option<Vec<u8>> {
    let end = lba.checked_add(count)?;
    if count == 0 || count > 256 || end > disk.capacity_sectors().ok()? {
        return None;
    }
    let mut out = alloc::vec![0u8; count as usize * SECTOR_SIZE];
    disk.read_at(lba, &mut out).ok()?;
    Some(out)
}

pub fn le16(b: &[u8], at: usize) -> u64 {
    u16::from_le_bytes([b[at], b[at + 1]]) as u64
}

pub fn le32(b: &[u8], at: usize) -> u64 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]) as u64
}

pub fn le64(b: &[u8], at: usize) -> u64 {
    le32(b, at) | (le32(b, at + 4) << 32)
}
