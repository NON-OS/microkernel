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

//! The receive side: the 512-entry standard ring buffers are posted on and
//! the 1024-entry return ring the part reports them on. Each standard slot
//! remembers its buffer, which goes back on the next free slot whatever
//! order slots come back in (as tg3_recycle_rx moves an address).

use crate::constants::layout::{RET_RING_AT, RX_BUFFERS, RX_BUFFER_LEN, STD_RING_AT};
use crate::constants::ring::{RX_RET_RING_SIZE, RX_STD_RING_SIZE};

use super::layout::RxBd;

pub struct RxQueue {
    rings_va: u64,
    pub std_ring_device_addr: u64,
    pub ret_ring_device_addr: u64,
    buffer_va: u64,
    buffer_device_addr: u64,
    /// The next standard slot to post on, the next return entry to read.
    pub std_prod: u16,
    pub ret_cons: u16,
    /// Which buffer each standard slot carries while the part holds it.
    pub(super) carries: [Option<u8>; RX_STD_RING_SIZE],
}

/*
 * SAFETY anchor for every unsafe in this module: the rings grant holds both
 * rings at STD_RING_AT and RET_RING_AT; each index is reduced into its ring.
 */
impl RxQueue {
    pub fn new(rings_va: u64, rings_dev: u64, buffer_va: u64, buffer_dev: u64) -> Self {
        Self {
            rings_va,
            std_ring_device_addr: rings_dev + STD_RING_AT as u64,
            ret_ring_device_addr: rings_dev + RET_RING_AT as u64,
            buffer_va,
            buffer_device_addr: buffer_dev,
            std_prod: 0,
            ret_cons: 0,
            carries: [None; RX_STD_RING_SIZE],
        }
    }

    pub(super) fn std_bd(&self, idx: usize) -> *mut RxBd {
        let idx = idx % RX_STD_RING_SIZE;
        unsafe { ((self.rings_va as usize + STD_RING_AT) as *mut RxBd).add(idx) }
    }

    pub(super) fn ret_bd(&self, idx: usize) -> *mut RxBd {
        let idx = idx % RX_RET_RING_SIZE;
        unsafe { ((self.rings_va as usize + RET_RING_AT) as *mut RxBd).add(idx) }
    }

    pub fn buffer_va(&self, buf: u8) -> u64 {
        self.buffer_va + (buf as usize % RX_BUFFERS * RX_BUFFER_LEN) as u64
    }

    pub(super) fn buffer_device_addr(&self, buf: u8) -> u64 {
        self.buffer_device_addr + (buf as usize % RX_BUFFERS * RX_BUFFER_LEN) as u64
    }
}
