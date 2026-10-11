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

//! The status block the part writes back by DMA: its first word, and
//! idx[0]'s return-ring producer and send-ring consumer (struct
//! tg3_hw_status, little-endian layout).

use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{fence, Ordering};

use crate::constants::desc::{STATUS_RX_PRODUCER, STATUS_TX_CONSUMER};

const SD_STATUS_UPDATED: u32 = 0x0000_0001;
const SD_STATUS_LINK_CHG: u32 = 0x0000_0002;

#[derive(Clone, Copy)]
pub struct Status {
    va: u64,
}

/*
 * SAFETY anchor for every unsafe here: `va` is the status block page of the
 * rings grant (setup/sequence.rs), and every offset read is inside its
 * TG3_HW_STATUS_SIZE bytes.
 */
impl Status {
    pub fn new(va: u64) -> Self {
        Self { va }
    }

    fn u16_at(&self, at: usize) -> u16 {
        let v = unsafe { read_volatile((self.va as usize + at) as *const u16) };
        // What the indices name was written before them.
        fence(Ordering::Acquire);
        v
    }

    pub fn rx_producer(&self) -> u16 {
        self.u16_at(STATUS_RX_PRODUCER)
    }

    pub fn tx_consumer(&self) -> u16 {
        self.u16_at(STATUS_TX_CONSUMER)
    }

    /// tg3_poll_link: a link change is taken once, the bit cleared in host
    /// memory and UPDATED left set, as Linux leaves it.
    pub fn take_link_change(&self) -> bool {
        let p = self.va as *mut u32;
        let word = unsafe { read_volatile(p) };
        if word & SD_STATUS_LINK_CHG == 0 {
            return false;
        }
        unsafe { write_volatile(p, (word & !SD_STATUS_LINK_CHG) | SD_STATUS_UPDATED) };
        true
    }

    pub fn clear(&self) {
        let base = self.va as *mut u8;
        unsafe { core::ptr::write_bytes(base, 0, crate::constants::ring::HW_STATUS_SIZE) };
    }
}
