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

//! The primary GPT header at LBA 1, held to the checks UEFI 2.10
//! 5.3.2 lists (as recalled, unverified) before a table is believed: signature, CRC, its own LBA.

use super::bytes::{le32, le64};
use super::crc32::crc32;

pub(crate) const SIGNATURE: &[u8; 8] = b"EFI PART";
/// The CRC covers `header_size` bytes, at least these 92.
const MIN_HEADER: usize = 92;
/// Only 128-byte entries are read (UEFI 2.10 allows 128 << n, as recalled).
pub(crate) const ENTRY_SIZE: usize = 128;
/// A larger array is not one any writer makes; reading it is refused.
pub(crate) const MAX_ENTRIES: u32 = 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Header {
    pub(crate) first_usable: u64,
    pub(crate) last_usable: u64,
    pub(crate) entries_lba: u64,
    pub(crate) entries: u32,
    pub(crate) entries_crc: u32,
}

impl Header {
    pub(crate) fn array_sectors(&self) -> u64 {
        (self.entries as u64 * ENTRY_SIZE as u64).div_ceil(512)
    }
}

/// `sector` is LBA 1 of a disk of `sectors`; `None` for no primary header.
pub(crate) fn parse(sector: &[u8], sectors: u64) -> Option<Header> {
    if sector.len() < 512 || &sector[..8] != SIGNATURE {
        return None;
    }
    let size = le32(sector, 12) as usize;
    if !(MIN_HEADER..=512).contains(&size) {
        return None;
    }
    let mut copy = [0u8; 512];
    copy[..size].copy_from_slice(&sector[..size]);
    copy[16..20].fill(0);
    let h = Header {
        first_usable: le64(sector, 40),
        last_usable: le64(sector, 48),
        entries_lba: le64(sector, 72),
        entries: le32(sector, 80),
        entries_crc: le32(sector, 88),
    };
    let fits = h.first_usable <= h.last_usable
        && h.last_usable < sectors
        && h.entries_lba >= 2
        && h.entries_lba.saturating_add(h.array_sectors()) <= sectors;
    let ok = crc32(&copy[..size]) == le32(sector, 16)
        && le64(sector, 24) == 1
        && le32(sector, 84) as usize == ENTRY_SIZE
        && (1..=MAX_ENTRIES).contains(&h.entries)
        && fits;
    ok.then_some(h)
}
