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

//! A block's integrity check: none, CRC-32, CRC-64 or SHA-256, as the
//! stream flags name it. Reserved check types are refused.

use super::crc32::crc32;
use super::crc64::crc64;

#[derive(Clone, Copy)]
pub enum Check {
    None,
    Crc32,
    Crc64,
    Sha256,
}

impl Check {
    pub fn from_flag(id: u8) -> Option<Check> {
        match id {
            0x00 => Some(Check::None),
            0x01 => Some(Check::Crc32),
            0x04 => Some(Check::Crc64),
            0x0A => Some(Check::Sha256),
            _ => None,
        }
    }

    pub fn size(self) -> usize {
        match self {
            Check::None => 0,
            Check::Crc32 => 4,
            Check::Crc64 => 8,
            Check::Sha256 => 32,
        }
    }

    /// `stored` is exactly `size()` bytes, as the block wrote it.
    pub fn holds(self, data: &[u8], stored: &[u8]) -> bool {
        match self {
            Check::None => stored.is_empty(),
            Check::Crc32 => stored == crc32(data).to_le_bytes(),
            Check::Crc64 => stored == crc64(data).to_le_bytes(),
            Check::Sha256 => stored == nonos_hash::sha256(data),
        }
    }
}
