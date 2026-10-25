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

use core::mem;
use core::slice;

use super::span::table_span;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct SdtHeader {
    pub signature: u32,
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}

impl SdtHeader {
    pub fn signature_bytes(&self) -> [u8; 4] {
        self.signature.to_le_bytes()
    }

    /// Whether the table's bytes sum to zero. A length under the header, or
    /// past MAX_TABLE_BYTES, longer than any firmware writes, is refused before
    /// a byte is summed: the sum would read whatever followed (L70, K11).
    pub fn validate_checksum(&self, table_ptr: *const u8) -> bool {
        if table_span(0, self.length, mem::size_of::<Self>(), u64::MAX).is_none() {
            return false;
        }
        // SAFETY: Caller ensures table_ptr is valid for self.length bytes,
        // as mapped_table checks against the direct map.
        unsafe {
            let bytes = slice::from_raw_parts(table_ptr, self.length as usize);
            bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b)) == 0
        }
    }

    pub fn data_length(&self) -> u32 {
        self.length.saturating_sub(mem::size_of::<Self>() as u32)
    }
}
