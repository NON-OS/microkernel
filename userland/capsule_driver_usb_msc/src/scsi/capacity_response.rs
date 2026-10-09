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

/// The READ CAPACITY(10) response length: the last LBA and the block length,
/// both big-endian 32-bit fields.
pub const CAPACITY_DATA_LEN: usize = 8;

/// The geometry a device reports in READ CAPACITY(10). `last_lba` is the address
/// of the final block (zero-based), so the block count is one more. `block_len`
/// is the real logical block size, which is how a 4Kn drive announces itself
/// instead of assuming 512.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Capacity {
    pub last_lba: u32,
    pub block_len: u32,
}

impl Capacity {
    /// Total addressable blocks. The reported LBA is the last block, so the
    /// count is one greater.
    pub fn block_count(&self) -> u64 {
        self.last_lba as u64 + 1
    }

    /// Total medium capacity in bytes.
    pub fn capacity_bytes(&self) -> u64 {
        self.block_count() * self.block_len as u64
    }
}

/// Decode the READ CAPACITY(10) response. Both fields are big-endian on the
/// wire. Returns None if the device returned fewer than eight bytes.
pub fn parse_capacity(raw: &[u8]) -> Option<Capacity> {
    if raw.len() < CAPACITY_DATA_LEN {
        return None;
    }
    Some(Capacity {
        last_lba: u32::from_be_bytes(raw[0..4].try_into().ok()?),
        block_len: u32::from_be_bytes(raw[4..8].try_into().ok()?),
    })
}

/// The READ CAPACITY(16) response length asked for. Bytes 0..8 are the last
/// LBA and 8..12 the block length, both big-endian; the rest (protection,
/// logical blocks per physical block) is not needed here.
pub const CAPACITY16_DATA_LEN: usize = 32;

/// The block count and block length READ CAPACITY(16) reports. None for a
/// reply too short to hold both fields, or a last LBA of all ones, whose
/// count does not fit.
pub fn parse_capacity16(raw: &[u8]) -> Option<(u64, u32)> {
    if raw.len() < 12 {
        return None;
    }
    let last_lba = u64::from_be_bytes(raw[0..8].try_into().ok()?);
    let block_len = u32::from_be_bytes(raw[8..12].try_into().ok()?);
    Some((last_lba.checked_add(1)?, block_len))
}
