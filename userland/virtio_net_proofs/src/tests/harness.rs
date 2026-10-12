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

//! One queue's ring and buffers, and the device's side of the ring.

use super::mem::Mem;
use crate::constants::{RX_BUFFER_LEN, RX_DESC_COUNT, TX_BUFFER_LEN, TX_DESC_COUNT};
use crate::constants::{VQ_AVAIL_OFFSET, VQ_REGION_SIZE, VQ_USED_OFFSET};
use crate::queue::{RxQueue, TxQueue};

/// Where the fake buffer grants sit for the device.
pub const RX_BUFS_AT: u64 = 0x4000_0000;
pub const TX_BUFS_AT: u64 = 0x5000_0000;

pub struct Dev {
    pub ring: Mem,
    pub bufs: Mem,
}

impl Dev {
    pub fn rx() -> Self {
        let bufs = Mem::new(RX_BUFFER_LEN as usize * RX_DESC_COUNT as usize);
        Self { ring: Mem::new(VQ_REGION_SIZE), bufs }
    }

    pub fn tx() -> Self {
        let bufs = Mem::new(TX_BUFFER_LEN as usize * TX_DESC_COUNT as usize);
        Self { ring: Mem::new(VQ_REGION_SIZE), bufs }
    }

    pub fn rx_queue(&self) -> RxQueue {
        RxQueue::new(self.ring.view(), self.bufs.view(), RX_BUFS_AT, RX_DESC_COUNT)
    }

    pub fn tx_queue(&self) -> TxQueue {
        TxQueue::new(self.ring.view(), self.bufs.view(), TX_BUFS_AT, TX_DESC_COUNT)
    }

    /// The device uses entry `n` (descriptor `id`, `len` bytes) and moves
    /// its index past it.
    pub fn complete(&self, n: u16, id: u32, len: u32) {
        let at = VQ_USED_OFFSET + 4 + 8 * usize::from(n % 256);
        self.ring.put(at, &id.to_le_bytes());
        self.ring.put(at + 4, &len.to_le_bytes());
        self.ring.put(VQ_USED_OFFSET + 2, &n.wrapping_add(1).to_le_bytes());
    }

    /// How many entries the driver has offered.
    pub fn avail_idx(&self) -> u16 {
        self.ring.u16_at(VQ_AVAIL_OFFSET + 2)
    }

    /// The descriptor offered as entry `n`.
    pub fn avail(&self, n: u16) -> u16 {
        self.ring.u16_at(VQ_AVAIL_OFFSET + 4 + 2 * usize::from(n % 256))
    }
}
