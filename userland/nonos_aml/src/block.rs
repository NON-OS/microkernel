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

//! A definition block: the table header of section 5.2.6 around the AML.

use crate::error::{Error, ErrorKind};

/// The header is 36 bytes; the AML term list starts right after it.
pub const HEADER_LEN: usize = 36;

/// A checked DSDT, SSDT or PSDT. Offsets everywhere else in this crate are
/// offsets into `bytes`, so a refusal can be found in a hexdump of the table.
#[derive(Clone, Copy, Debug)]
pub struct Block<'a> {
    pub bytes: &'a [u8],
}

impl<'a> Block<'a> {
    /// Takes the table as handed over, trimmed to its own length field, and
    /// refuses one whose checksum fails or that is larger than `max_len`.
    pub fn new(table: &'a [u8], max_len: usize) -> Result<Self, Error> {
        if table.len() < HEADER_LEN {
            return Err(Error::new(ErrorKind::Header, 0));
        }
        let len = u32::from_le_bytes([table[4], table[5], table[6], table[7]]) as usize;
        if len < HEADER_LEN || len > table.len() {
            return Err(Error::new(ErrorKind::Header, 4));
        }
        if len > max_len {
            return Err(Error::new(ErrorKind::Size, 4));
        }
        let bytes = &table[..len];
        if bytes.iter().fold(0u8, |s, b| s.wrapping_add(*b)) != 0 {
            return Err(Error::new(ErrorKind::Checksum, 9));
        }
        Ok(Self { bytes })
    }

    pub fn signature(&self) -> [u8; 4] {
        [self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]]
    }

    pub fn revision(&self) -> u8 {
        self.bytes[8]
    }

    /// Section 5.2.11.1: a revision below 2 means integers are 32 bits wide.
    pub fn wide_integers(&self) -> bool {
        self.revision() >= 2
    }

    pub fn oem_table_id(&self) -> [u8; 8] {
        let mut id = [0u8; 8];
        id.copy_from_slice(&self.bytes[16..24]);
        id
    }

    /// The table offsets the AML term list spans.
    pub fn aml(&self) -> (u32, u32) {
        (HEADER_LEN as u32, self.bytes.len() as u32)
    }
}
