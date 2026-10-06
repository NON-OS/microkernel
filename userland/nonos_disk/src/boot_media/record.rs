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
 * The record the loader leaves: `BootMedia` in the loader's and the
 * kernel's handoff types, forty bytes, little-endian, read here by offset
 * so its layout is spelled once on this side.
 *
 *   0..4 partition number   4 table   5 signature type   6..8 reserved
 *   8..16 start LBA   16..24 size in sectors   24..40 signature
 */

use super::read::{le32, le64};

pub const BOOT_MEDIA_LEN: usize = 40;
pub const TABLE_MBR: u8 = 1;
pub const TABLE_GPT: u8 = 2;
pub const SIGNATURE_MBR: u8 = 1;
pub const SIGNATURE_GUID: u8 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BootPartition {
    /* From 1, as the firmware counts. */
    pub number: u32,
    pub table: u8,
    pub signature_type: u8,
    pub start_lba: u64,
    pub size_lba: u64,
    /* A GPT partition GUID as stored, or an MBR disk signature in 0..4. */
    pub signature: [u8; 16],
}

impl BootPartition {
    /* `None` for a record of another length or one naming no table. */
    pub fn parse(b: &[u8]) -> Option<BootPartition> {
        if b.len() != BOOT_MEDIA_LEN || !matches!(b[4], TABLE_MBR | TABLE_GPT) {
            return None;
        }
        let mut signature = [0u8; 16];
        signature.copy_from_slice(&b[24..40]);
        Some(BootPartition {
            number: le32(b, 0) as u32,
            table: b[4],
            signature_type: b[5],
            start_lba: le64(b, 8),
            size_lba: le64(b, 16),
            signature,
        })
    }

    pub fn to_bytes(&self) -> [u8; BOOT_MEDIA_LEN] {
        let mut b = [0u8; BOOT_MEDIA_LEN];
        b[0..4].copy_from_slice(&self.number.to_le_bytes());
        b[4] = self.table;
        b[5] = self.signature_type;
        b[8..16].copy_from_slice(&self.start_lba.to_le_bytes());
        b[16..24].copy_from_slice(&self.size_lba.to_le_bytes());
        b[24..40].copy_from_slice(&self.signature);
        b
    }
}
