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

//! Clearing the rings, and posting one frame: address and end-of-packet with a completion request
//! first, then the length with the generation bit, which hands the slot to
//! the device. The caller writes TXPROD with the value returned.

use core::ptr::{addr_of_mut, write_volatile};
use core::sync::atomic::{fence, Ordering};

use super::desc::{Comp, Desc};
use super::tx::TxRing;
use crate::constants::ring::{
    INIT_GEN, LEN_MASK, TXD_CQ, TXD_EOP, TXD_GEN_SHIFT, TX_COMP, TX_RING,
};

// SAFETY anchor: see tx.rs.
impl TxRing {
    /// Both rings empty and every index back at its start, before the
    /// device is told where they are.
    pub fn reset(&mut self) {
        for i in 0..TX_RING as u16 {
            unsafe { core::ptr::write_volatile(self.desc(i), Desc::default()) };
        }
        for i in 0..TX_COMP as u16 {
            unsafe { core::ptr::write_volatile(self.comp(i), Comp::default()) };
        }
        (self.fill, self.clean, self.comp_next) = (0, 0, 0);
        (self.gen, self.comp_gen) = (INIT_GEN, INIT_GEN);
    }

    pub fn post(&mut self, len: u16) -> u32 {
        let d = unsafe { self.desc(self.fill) };
        unsafe {
            write_volatile(addr_of_mut!((*d).addr), self.buffer_dev(self.fill));
            write_volatile(addr_of_mut!((*d).d3), TXD_EOP | TXD_CQ);
        }
        // The frame bytes and the fields above are plain stores; the device
        // may read them all the moment the generation bit matches its own.
        fence(Ordering::Release);
        let d2 = (len as u32 & LEN_MASK) | self.gen << TXD_GEN_SHIFT;
        unsafe { write_volatile(addr_of_mut!((*d).d2), d2) };
        self.fill = (self.fill + 1) % TX_RING as u16;
        if self.fill == 0 {
            self.gen ^= 1;
        }
        self.fill as u32
    }
}
