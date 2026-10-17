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

//! A GPT header, primary or backup, believed only when its signature,
//! size, CRC, own LBA and ranges hold (UEFI 2.10, 5.3.2, as recalled,
//! unverified): "EFI PART"; HeaderSize at least its 92 bytes of fields and
//! at most the block it sits in, 512 here since only 512-byte sectors are
//! read; the CRC over HeaderSize bytes with its own field zeroed; MyLBA the
//! LBA read; FirstUsableLBA at most LastUsableLBA, both inside the disk;
//! the array as array.rs shapes it and which.rs places it.

use super::array::{shaped, within};
use super::bytes::{le32, le64};
use super::crc32::crc32;
pub(crate) use super::fields::Header;
use super::which::Which;

pub(crate) const SIGNATURE: &[u8; 8] = b"EFI PART";
const MIN_HEADER: usize = 92;
const SECTOR: usize = 512;

/// `sector` is where `which` sits on a disk of `sectors`; `None` for no
/// header of that kind there.
pub(crate) fn parse_as(sector: &[u8], sectors: u64, which: Which) -> Option<Header> {
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
        disk_guid: sector[56..72].try_into().ok()?,
        first_usable: le64(sector, 40),
        last_usable: le64(sector, 48),
        entries_lba: le64(sector, 72),
        entries: le32(sector, 80),
        entry_size: le32(sector, 84),
        entries_crc: le32(sector, 88),
    };
    let ranges = h.first_usable <= h.last_usable && h.last_usable < sectors;
    let (lo, hi) = which.array_room(h.first_usable, h.last_usable, sectors);
    let n = (h.entries, h.entry_size);
    let array = shaped(n.0, n.1) && within(h.entries_lba, n.0, n.1, lo, hi);
    let own = le64(sector, 24) == which.my_lba(sectors) && which.names_other(le64(sector, 32));
    let ok = crc32(&copy[..size]) == le32(sector, 16) && own;
    (ok && ranges && array).then_some(h)
}
