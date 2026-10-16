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

//! The primary GPT header at LBA 1, believed only when its signature,
//! size, CRC, own LBA and ranges hold (UEFI 2.10, 5.3.2, as recalled,
//! unverified): "EFI PART"; HeaderSize at least its 92 bytes of fields and
//! at most the block it sits in, 512 here since only 512-byte sectors are
//! read; the CRC over HeaderSize bytes with its own field zeroed; MyLBA the
//! LBA read, 1; FirstUsableLBA at most LastUsableLBA, both inside the
//! disk; the entry array as array.rs holds it.

use super::array::{array_fits, array_sectors};
use super::bytes::{le32, le64};
use super::crc32::crc32;

pub(crate) const SIGNATURE: &[u8; 8] = b"EFI PART";
const MIN_HEADER: usize = 92;
const SECTOR: usize = 512;
const MY_LBA: u64 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Header {
    pub(crate) first_usable: u64,
    pub(crate) last_usable: u64,
    pub(crate) entries_lba: u64,
    pub(crate) entries: u32,
    pub(crate) entry_size: u32,
    pub(crate) entries_crc: u32,
}

impl Header {
    pub(crate) fn array_sectors(&self) -> u64 {
        array_sectors(self.entries, self.entry_size)
    }
}

/// `sector` is LBA 1 of a disk of `sectors`; `None` for no primary header.
pub(crate) fn parse(sector: &[u8], sectors: u64) -> Option<Header> {
    if sector.len() < SECTOR || &sector[..8] != SIGNATURE {
        return None;
    }
    let size = le32(sector, 12) as usize;
    if !(MIN_HEADER..=SECTOR).contains(&size) {
        return None;
    }
    let mut copy = [0u8; SECTOR];
    copy[..size].copy_from_slice(&sector[..size]);
    copy[16..20].fill(0);
    let h = Header {
        first_usable: le64(sector, 40),
        last_usable: le64(sector, 48),
        entries_lba: le64(sector, 72),
        entries: le32(sector, 80),
        entry_size: le32(sector, 84),
        entries_crc: le32(sector, 88),
    };
    let ranges = h.first_usable <= h.last_usable && h.last_usable < sectors;
    let array = array_fits(h.entries_lba, h.entries, h.entry_size, h.first_usable);
    let ok = crc32(&copy[..size]) == le32(sector, 16) && le64(sector, 24) == MY_LBA;
    (ok && ranges && array).then_some(h)
}
