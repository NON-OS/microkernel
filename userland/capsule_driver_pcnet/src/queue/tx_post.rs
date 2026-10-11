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

//! Posting one frame: the descriptor at `tail` is filled, then OWN with STP
//! and ENP is stored last. The caller sets CSR0.TDMD so the part looks now
//! rather than at its next poll.

use core::ptr::{addr_of_mut, write_volatile};
use core::sync::atomic::{fence, Ordering};

use crate::constants::queue::{bcnt, ENP, OWN, STP, TX_DESC_COUNT};

use super::tx::TxRing;

// SAFETY anchor: `tail` stays below TX_DESC_COUNT, see tx.rs.
impl TxRing {
    /// Empty every descriptor before the part is told where the ring is.
    pub fn clear(&mut self) {
        self.tail = 0;
        self.clean = 0;
        for idx in 0..TX_DESC_COUNT as u16 {
            unsafe { write_volatile(self.descriptor(idx), Default::default()) };
        }
    }

    pub fn post(&mut self, len: u16) -> u16 {
        let idx = self.tail;
        let d = unsafe { self.descriptor(idx) };
        unsafe {
            write_volatile(addr_of_mut!((*d).addr), self.buffer_device_addr(idx));
            write_volatile(addr_of_mut!((*d).bcnt), bcnt(len as usize));
            write_volatile(addr_of_mut!((*d).misc), 0);
        }
        // The frame bytes and the fields above are plain stores; the part may
        // read them all the moment OWN is visible.
        fence(Ordering::Release);
        unsafe { write_volatile(addr_of_mut!((*d).status), OWN | STP | ENP) };
        self.tail = (self.tail + 1) % TX_DESC_COUNT as u16;
        idx
    }
}
