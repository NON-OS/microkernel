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

//! Posting one frame on the TX ring: the descriptor for the slot at `tail`
//! is written field by field and `tail` moves on. The caller writes TDT.

use core::ptr::{addr_of_mut, write_volatile};

use crate::constants::queue::{TX_CMD_EOP, TX_CMD_IFCS, TX_CMD_RS, TX_DESC_COUNT};

use super::tx::TxRing;

// SAFETY anchor: see tx.rs; `tail` stays below TX_DESC_COUNT.
impl TxRing {
    pub fn post(&mut self, len: u16) -> u16 {
        let idx = self.tail;
        let desc = unsafe { self.descriptor(idx) };
        /*
         * The part reads the descriptor by DMA once the tail moves, so the
         * fields are stored in program order and none is left in a register.
         */
        unsafe {
            write_volatile(addr_of_mut!((*desc).buffer_addr), self.buffer_phys(idx));
            write_volatile(addr_of_mut!((*desc).length), len);
            write_volatile(addr_of_mut!((*desc).cmd), TX_CMD_EOP | TX_CMD_IFCS | TX_CMD_RS);
            write_volatile(addr_of_mut!((*desc).status), 0);
        }
        self.tail = (self.tail + 1) % (TX_DESC_COUNT as u16);
        idx
    }
}
