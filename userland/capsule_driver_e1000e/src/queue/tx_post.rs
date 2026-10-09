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

//! Posting and reclaiming. `post` programs the next descriptor with
//! `EOP|IFCS|RS`; `reclaim` walks `clean` forward over descriptors the part
//! marked DD, and `full` refuses a post that would land on one it still owns.
//! With the link down the part holds descriptors and sets no DD, so a slot is
//! reusable only once `reclaim` has seen it done. One slot stays empty, or a
//! full ring would move TDT onto TDH, which the part reads as an empty one.

use core::ptr::{addr_of, addr_of_mut, read_volatile, write_volatile};
use core::sync::atomic::{fence, Ordering};

use crate::constants::queue::{TX_CMD_EOP, TX_CMD_IFCS, TX_CMD_RS, TX_DESC_COUNT, TX_STATUS_DD};

use super::tx::TxRing;

impl TxRing {
    pub fn post(&mut self, len: u16) -> u16 {
        let idx = self.tail;
        // SAFETY: `tail` is kept below TX_DESC_COUNT by the modulo below. The
        // part reads the descriptor by DMA once the tail moves, so each field
        // is stored to memory in program order.
        unsafe {
            let desc = self.descriptor(idx);
            write_volatile(addr_of_mut!((*desc).buffer_addr), self.buffer_phys(idx));
            write_volatile(addr_of_mut!((*desc).length), len);
            write_volatile(addr_of_mut!((*desc).cmd), TX_CMD_EOP | TX_CMD_IFCS | TX_CMD_RS);
            write_volatile(addr_of_mut!((*desc).status), 0);
        }
        self.tail = (self.tail + 1) % (TX_DESC_COUNT as u16);
        idx
    }

    pub fn done(&self, idx: u16) -> bool {
        // SAFETY: callers pass `clean` or a posted index, both below the count.
        let status = unsafe { read_volatile(addr_of!((*self.descriptor(idx)).status)) };
        fence(Ordering::Acquire);
        status & TX_STATUS_DD != 0
    }

    pub fn reclaim(&mut self) {
        while self.clean != self.tail && self.done(self.clean) {
            self.clean = (self.clean + 1) % (TX_DESC_COUNT as u16);
        }
    }

    pub fn full(&self) -> bool {
        (self.tail + 1) % (TX_DESC_COUNT as u16) == self.clean
    }
}
