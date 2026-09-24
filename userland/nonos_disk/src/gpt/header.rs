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

//! The 92-byte GPT header, primary or backup. The two differ only in which
//! LBA they call their own, which they call the other's, and where the
//! entry array is; the CRC covers the 92 bytes with its own field zero.

use alloc::vec::Vec;

use super::layout::{Layout, ARRAY_SECTORS, ENTRY_COUNT, ENTRY_SIZE};
use crate::crc32::crc32;
use crate::guid::Guid;
use crate::sink::SECTOR_SIZE;

pub const SIGNATURE: &[u8; 8] = b"EFI PART";
const REVISION: u32 = 0x0001_0000;
const HEADER_SIZE: u32 = 92;

#[derive(Clone, Copy)]
pub enum Which {
    Primary,
    Backup,
}

pub fn build(layout: &Layout, disk_guid: Guid, array_crc: u32, which: Which) -> Vec<u8> {
    let (my_lba, alt_lba, array_lba) = match which {
        Which::Primary => (1u64, layout.backup_header_lba, 2u64),
        Which::Backup => (layout.backup_header_lba, 1u64, layout.backup_array_lba),
    };
    let mut h = [0u8; HEADER_SIZE as usize];
    h[0..8].copy_from_slice(SIGNATURE);
    h[8..12].copy_from_slice(&REVISION.to_le_bytes());
    h[12..16].copy_from_slice(&HEADER_SIZE.to_le_bytes());
    h[24..32].copy_from_slice(&my_lba.to_le_bytes());
    h[32..40].copy_from_slice(&alt_lba.to_le_bytes());
    h[40..48].copy_from_slice(&(2 + ARRAY_SECTORS).to_le_bytes());
    h[48..56].copy_from_slice(&(layout.backup_array_lba - 1).to_le_bytes());
    h[56..72].copy_from_slice(&disk_guid.0);
    h[72..80].copy_from_slice(&array_lba.to_le_bytes());
    h[80..84].copy_from_slice(&ENTRY_COUNT.to_le_bytes());
    h[84..88].copy_from_slice(&ENTRY_SIZE.to_le_bytes());
    h[88..92].copy_from_slice(&array_crc.to_le_bytes());
    let crc = crc32(&h);
    h[16..20].copy_from_slice(&crc.to_le_bytes());
    let mut sector = alloc::vec![0u8; SECTOR_SIZE];
    sector[..h.len()].copy_from_slice(&h);
    sector
}
