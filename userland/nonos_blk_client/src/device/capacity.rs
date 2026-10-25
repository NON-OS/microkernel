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

//! How big the disk is: the driver's capacity in its own blocks, and the
//! size in bytes the list shows.

use super::handle::BlockDevice;
use crate::driver::CAPACITY_BODY_MAX;
use crate::error::BlkError;
use crate::wire::{call, decode_reply, HDR_LEN, STATUS_LEN};

impl BlockDevice {
    /// The capacity as the driver gives it, in the disk's own blocks.
    pub fn capacity_blocks(&self) -> Result<u64, BlkError> {
        let ops = self.driver.ops();
        // Sized for the longest reply; decode_reply cuts it to what came.
        let mut rx = [0u8; HDR_LEN + STATUS_LEN + CAPACITY_BODY_MAX];
        let (n, id) = call(self.port, self.driver.magic(), ops.capacity, self.unit, &[], &mut rx)?;
        let body = decode_reply(&rx, n, self.driver.magic(), ops.capacity, id)?;
        if body.len() < 8 {
            return Err(BlkError::BadLength);
        }
        let mut w = [0u8; 8];
        w.copy_from_slice(&body[..8]);
        Ok(u64::from_le_bytes(w))
    }

    /// Saturated: the size is the driver's word, and a size shown as a few
    /// bytes because it wrapped would misdescribe the disk.
    pub fn bytes(&self) -> u64 {
        self.sectors.saturating_mul(crate::wire::SECTOR_SIZE as u64)
    }
}
