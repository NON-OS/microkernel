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

//! TX ring state. `post` writes the next advanced data descriptor and moves
//! the software tail; the caller then writes TDT. Reclaim and the full test
//! are in tx_reclaim.rs.

use core::ptr::{addr_of_mut, write_volatile};

use crate::constants::queue::{TX_BUFFER_LEN, TX_DESC_COUNT};

use super::layout::TxDesc;
use super::tx_encode::{cmd_type_len, olinfo_status};

pub struct TxRing {
    pub ring_user_va: u64,
    pub buffer_user_va: u64,
    pub buffer_device_addr: u64,
    pub tail: u16,
    /// Oldest descriptor not yet seen done; `clean == tail` is an empty ring.
    pub clean: u16,
}

/*
 * SAFETY anchor for every unsafe in this file: `ring_user_va` is the broker
 * DMA grant taken in `setup::dma`, TX_DESC_COUNT 16-byte descriptors long,
 * and `tail` is held below TX_DESC_COUNT by `post`.
 */
impl TxRing {
    pub fn new(ring_user_va: u64, buffer_user_va: u64, buffer_device_addr: u64) -> Self {
        Self { ring_user_va, buffer_user_va, buffer_device_addr, tail: 0, clean: 0 }
    }

    /// # Safety
    /// `idx` is below TX_DESC_COUNT and the ring is the grant taken in setup.
    pub unsafe fn descriptor(&self, idx: u16) -> *mut TxDesc {
        (self.ring_user_va as *mut TxDesc).add(idx as usize)
    }

    pub fn buffer_phys(&self, idx: u16) -> u64 {
        self.buffer_device_addr + (idx as u64) * (TX_BUFFER_LEN as u64)
    }

    pub fn buffer_va(&self, idx: u16) -> u64 {
        self.buffer_user_va + (idx as u64) * (TX_BUFFER_LEN as u64)
    }

    /// The part reads the descriptor by DMA once TDT moves, so each field is
    /// stored in program order. olinfo_status carries PAYLEN from bit 14, so
    /// the DD bit the part writes back there starts clear.
    pub fn post(&mut self, len: u16) -> u16 {
        let idx = self.tail;
        let desc = unsafe { self.descriptor(idx) };
        unsafe {
            write_volatile(addr_of_mut!((*desc).buffer_addr), self.buffer_phys(idx));
            write_volatile(addr_of_mut!((*desc).cmd_type_len), cmd_type_len(len));
            write_volatile(addr_of_mut!((*desc).olinfo_status), olinfo_status(len));
        }
        self.tail = (self.tail + 1) % (TX_DESC_COUNT as u16);
        idx
    }
}
