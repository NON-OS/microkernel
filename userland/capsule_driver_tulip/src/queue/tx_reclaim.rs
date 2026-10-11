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

//! Completion on the transmit ring: the part clears OWN on a descriptor it
//! is done with, sent or abandoned after its retries. Either way the slot is
//! free again; a transmit error is the wire's, not the caller's.

use core::ptr::{addr_of, read_volatile};
use core::sync::atomic::{fence, Ordering};

use crate::constants::queue::{OWN, TX_DESC_COUNT};

use super::tx::TxRing;

// SAFETY anchor: `clean` stays below TX_DESC_COUNT, see tx.rs.
impl TxRing {
    pub fn done(&self, idx: u16) -> bool {
        let status = unsafe { read_volatile(addr_of!((*self.descriptor(idx)).status)) };
        fence(Ordering::Acquire);
        status & OWN == 0
    }

    pub fn reclaim(&mut self) {
        while self.clean != self.tail && self.done(self.clean) {
            self.clean = (self.clean + 1) % TX_DESC_COUNT as u16;
        }
    }

    pub fn full(&self) -> bool {
        (self.tail + 1) % TX_DESC_COUNT as u16 == self.clean
    }
}
