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

//! TX ring state over its DMA grants: where each descriptor and each slot's
//! buffer is. Posting and reclaiming are in `tx_post`.

use crate::constants::queue::TX_BUFFER_LEN;

use super::layout::TxDesc;

pub struct TxRing {
    pub ring_user_va: u64,
    pub buffer_user_va: u64,
    pub buffer_device_addr: u64,
    pub tail: u16,
    /// Oldest descriptor not yet seen done; `clean == tail` is an empty ring.
    pub clean: u16,
}

impl TxRing {
    pub fn new(ring_user_va: u64, buffer_user_va: u64, buffer_device_addr: u64) -> Self {
        Self { ring_user_va, buffer_user_va, buffer_device_addr, tail: 0, clean: 0 }
    }

    /// # Safety
    ///
    /// `idx` is below the descriptor count; the ring memory is the DMA grant
    /// taken in setup, `TX_DESC_COUNT` descriptors long.
    pub unsafe fn descriptor(&self, idx: u16) -> *mut TxDesc {
        (self.ring_user_va as *mut TxDesc).add(idx as usize)
    }

    pub fn buffer_phys(&self, idx: u16) -> u64 {
        self.buffer_device_addr + (idx as u64) * (TX_BUFFER_LEN as u64)
    }

    pub fn buffer_va(&self, idx: u16) -> u64 {
        self.buffer_user_va + (idx as u64) * (TX_BUFFER_LEN as u64)
    }
}
