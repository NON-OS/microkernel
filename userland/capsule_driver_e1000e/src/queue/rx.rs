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

//! RX ring state over its DMA grants: where each descriptor and each slot's
//! buffer is. Taking a received frame is in `rx_consume`.

use crate::constants::queue::RX_BUFFER_LEN;

use super::layout::RxDesc;

pub struct RxRing {
    pub ring_user_va: u64,
    pub buffer_user_va: u64,
    pub buffer_device_addr: u64,
    pub head: u16,
}

impl RxRing {
    pub fn new(ring_user_va: u64, buffer_user_va: u64, buffer_device_addr: u64) -> Self {
        Self { ring_user_va, buffer_user_va, buffer_device_addr, head: 0 }
    }

    /// # Safety
    ///
    /// `idx` is below the descriptor count; the ring memory is the DMA grant
    /// taken in setup, `RX_DESC_COUNT` descriptors long.
    pub unsafe fn descriptor(&self, idx: u16) -> *mut RxDesc {
        (self.ring_user_va as *mut RxDesc).add(idx as usize)
    }

    pub fn buffer_phys(&self, idx: u16) -> u64 {
        self.buffer_device_addr + (idx as u64) * (RX_BUFFER_LEN as u64)
    }

    pub fn buffer_va(&self, idx: u16) -> u64 {
        self.buffer_user_va + (idx as u64) * (RX_BUFFER_LEN as u64)
    }
}
