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

//! The register as read: revision, device type, capacity and partition.

use super::offsets::{DEVICE_TYPE, EXT_CSD_LEN, EXT_CSD_REV, PARTITION_CONFIG, SEC_COUNT};
use super::values::PARTITION_ACCESS_MASK;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ExtCsd {
    pub raw: [u8; EXT_CSD_LEN],
}

impl core::fmt::Debug for ExtCsd {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ExtCsd")
            .field("rev", &self.rev())
            .field("sec_count", &self.sec_count())
            .field("device_type", &self.device_type())
            .field("partition_config", &self.partition_config())
            .finish()
    }
}

impl ExtCsd {
    pub const fn new(raw: [u8; EXT_CSD_LEN]) -> Self {
        Self { raw }
    }

    pub const fn rev(&self) -> u8 {
        self.raw[EXT_CSD_REV]
    }

    pub const fn device_type(&self) -> u8 {
        self.raw[DEVICE_TYPE]
    }

    /// SEC_COUNT, bytes 215:212 little endian: the user area in 512-byte
    /// sectors (for a card larger than 2 GB, in sector mode).
    pub const fn sec_count(&self) -> u32 {
        u32::from_le_bytes([
            self.raw[SEC_COUNT],
            self.raw[SEC_COUNT + 1],
            self.raw[SEC_COUNT + 2],
            self.raw[SEC_COUNT + 3],
        ])
    }

    pub const fn partition_config(&self) -> u8 {
        self.raw[PARTITION_CONFIG as usize]
    }

    /// The partition reads and writes go to: 0 user area, 1 and 2 boot,
    /// 3 RPMB, 4 to 7 general purpose.
    pub const fn partition_access(&self) -> u8 {
        self.partition_config() & PARTITION_ACCESS_MASK
    }
}
