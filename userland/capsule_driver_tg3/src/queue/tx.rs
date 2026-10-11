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

//! The 512-entry send ring. At most TX_BUFFERS - 1 frames are in flight,
//! each in the 2 KiB slot its ring index names modulo TX_BUFFERS, so two
//! frames in flight never share a slot.

use core::ptr::write_bytes;

use crate::constants::layout::{TX_BUFFERS, TX_RING_AT};
use crate::constants::ring::TX_RING_SIZE;

use super::layout::TxBd;

pub struct TxRing {
    rings_va: u64,
    pub ring_device_addr: u64,
    pub(super) buffer_va: u64,
    pub(super) buffer_device_addr: u64,
    /// The next entry to fill, and the oldest the part has not finished.
    pub prod: u16,
    pub cons: u16,
}

/*
 * SAFETY anchor for every unsafe here: the rings grant holds TX_RING_SIZE
 * send BDs at TX_RING_AT, and `bd` reduces every index into it.
 */
impl TxRing {
    pub fn new(rings_va: u64, rings_dev: u64, buffer_va: u64, buffer_dev: u64) -> Self {
        let ring_device_addr = rings_dev + TX_RING_AT as u64;
        Self {
            rings_va,
            ring_device_addr,
            buffer_va,
            buffer_device_addr: buffer_dev,
            prod: 0,
            cons: 0,
        }
    }

    pub(super) fn bd(&self, idx: usize) -> *mut TxBd {
        let idx = idx % TX_RING_SIZE;
        unsafe { ((self.rings_va as usize + TX_RING_AT) as *mut TxBd).add(idx) }
    }

    pub fn init(&mut self) {
        unsafe { write_bytes(self.bd(0), 0, TX_RING_SIZE) };
        (self.prod, self.cons) = (0, 0);
    }

    pub fn in_flight(&self) -> usize {
        (self.prod as usize + TX_RING_SIZE - self.cons as usize) % TX_RING_SIZE
    }

    pub fn full(&self) -> bool {
        self.in_flight() >= TX_BUFFERS - 1
    }
}
