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

//! The CSD fields the driver uses, and a byte-mode card's capacity.

use super::r2::bits;

/// The CSD fields the driver uses (JEDEC eMMC 5.1, 7.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Csd {
    pub structure: u8,
    /// SPEC_VERS: 4 means EXT_CSD and the version 4 commands exist.
    pub spec_vers: u8,
    pub tran_speed: u8,
    pub read_bl_len: u8,
    pub c_size: u32,
    pub c_size_mult: u8,
}

/// C_SIZE all ones: the capacity is in EXT_CSD SEC_COUNT instead.
pub const C_SIZE_EXT: u32 = 0xfff;

impl Csd {
    pub const fn parse(reg: u128) -> Self {
        Self {
            structure: bits(reg, 127, 126) as u8,
            spec_vers: bits(reg, 125, 122) as u8,
            tran_speed: bits(reg, 103, 96) as u8,
            read_bl_len: bits(reg, 83, 80) as u8,
            c_size: bits(reg, 73, 62),
            c_size_mult: bits(reg, 49, 47) as u8,
        }
    }

    /// The capacity of a byte-addressed card in 512-byte sectors:
    /// (C_SIZE + 1) x 2^(C_SIZE_MULT + 2) blocks of 2^READ_BL_LEN bytes.
    /// None when the CSD defers to EXT_CSD or names an impossible block.
    pub const fn sectors(&self) -> Option<u64> {
        if self.c_size == C_SIZE_EXT || self.read_bl_len < 9 || self.read_bl_len > 11 {
            return None;
        }
        let blocks = (self.c_size as u64 + 1) << (self.c_size_mult as u32 + 2);
        Some((blocks << self.read_bl_len) / 512)
    }
}
