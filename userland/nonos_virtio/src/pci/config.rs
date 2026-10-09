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

//! The first 256 bytes of config space, read once per bring-up attempt.
//!
//! The capability walk then runs over plain bytes, which is what lets it be
//! proved against hostile layouts on the host. Every accessor is bounds
//! checked and answers `None` past the end, so no capability field, however
//! it is placed, reads outside the snapshot.

use crate::broker::Broker;

/// The standard header plus the capability area the walk can reach.
pub const CONFIG_SPACE_LEN: usize = 256;

#[derive(Clone)]
pub struct ConfigSpace {
    bytes: [u8; CONFIG_SPACE_LEN],
}

impl ConfigSpace {
    pub const fn from_bytes(bytes: [u8; CONFIG_SPACE_LEN]) -> Self {
        Self { bytes }
    }

    /// Read the snapshot through the broker, one dword at a time (the only
    /// width every config path accepts at every aligned offset). `None` if
    /// any read is refused: a partial snapshot would parse as zeros.
    pub fn read(broker: &mut impl Broker) -> Option<Self> {
        let mut bytes = [0u8; CONFIG_SPACE_LEN];
        for (i, dword) in bytes.chunks_exact_mut(4).enumerate() {
            let value = broker.config_read32((i * 4) as u32)?;
            dword.copy_from_slice(&value.to_le_bytes());
        }
        Some(Self { bytes })
    }

    pub fn u8_at(&self, off: usize) -> Option<u8> {
        self.bytes.get(off).copied()
    }

    pub fn u16_at(&self, off: usize) -> Option<u16> {
        let b = self.bytes.get(off..off.checked_add(2)?)?;
        Some(u16::from_le_bytes([b[0], b[1]]))
    }

    pub fn u32_at(&self, off: usize) -> Option<u32> {
        let b = self.bytes.get(off..off.checked_add(4)?)?;
        Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }
}
