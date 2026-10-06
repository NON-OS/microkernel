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

//! Completion side of the TX ring. A slot is reusable only once the part has
//! written DD back into it (igc_clean_tx_irq tests IGC_TXD_STAT_DD in
//! wb.status): with the link down the part holds descriptors and writes
//! nothing. One slot always stays empty, or a full ring would move TDT onto
//! TDH, which the part reads as an empty one.

use core::ptr::{addr_of, read_volatile};
use core::sync::atomic::{fence, Ordering};

use crate::constants::queue::TX_DESC_COUNT;
use crate::constants::tx_bits::TXD_STAT_DD;

use super::tx::TxRing;

/*
 * SAFETY anchor for every unsafe in this file: `clean` only moves modulo
 * TX_DESC_COUNT, so `descriptor(self.clean)` is inside the ring grant.
 */
impl TxRing {
    pub fn done(&self, idx: u16) -> bool {
        let status = unsafe { read_volatile(addr_of!((*self.descriptor(idx)).olinfo_status)) };
        fence(Ordering::Acquire);
        status & TXD_STAT_DD != 0
    }

    /// Advance `clean` over every descriptor the part has finished, in order.
    pub fn reclaim(&mut self) {
        while self.clean != self.tail && self.done(self.clean) {
            self.clean = (self.clean + 1) % (TX_DESC_COUNT as u16);
        }
    }

    /// Whether posting one more descriptor would reach one the part owns.
    pub fn full(&self) -> bool {
        (self.tail + 1) % (TX_DESC_COUNT as u16) == self.clean
    }
}
