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

use crate::constants::{
    FLUSH_MS, REQUEST_MS, SECTOR_SIZE, VIRTIO_BLK_T_FLUSH, VIRTIO_BLK_T_IN, VIRTIO_BLK_T_OUT,
};

/// The three requests the driver makes. DISCARD, WRITE_ZEROES and the
/// other request types are never negotiated (plan/57 names them their own
/// unit), so the device is never sent one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Read,
    Write,
    Flush,
}

impl Direction {
    pub(super) fn req_type(self) -> u32 {
        match self {
            Direction::Read => VIRTIO_BLK_T_IN,
            Direction::Write => VIRTIO_BLK_T_OUT,
            Direction::Flush => VIRTIO_BLK_T_FLUSH,
        }
    }

    /// Bytes the data descriptor carries: none for a flush.
    pub(super) fn data_bytes(self, nsectors: u32) -> Option<u32> {
        match self {
            Direction::Flush => Some(0),
            _ => nsectors.checked_mul(SECTOR_SIZE as u32),
        }
    }

    /// The device-writable bytes of the chain, which its used element must
    /// report: a read's data, then the status byte.
    pub(super) fn writable(self, data: u32) -> u32 {
        match self {
            Direction::Read => data.saturating_add(1),
            Direction::Write | Direction::Flush => 1,
        }
    }

    /// How long the device may take with it.
    pub fn budget_ms(self) -> u64 {
        match self {
            Direction::Flush => FLUSH_MS,
            Direction::Read | Direction::Write => REQUEST_MS,
        }
    }
}
