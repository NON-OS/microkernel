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

//! Transmit completions: each names the end-of-packet slot it finished,
//! and every slot up to it is free again. The device completes in order.

use core::ptr::{addr_of, read_volatile};
use core::sync::atomic::{fence, Ordering};

use super::desc::completed;
use super::tx::TxRing;
use crate::constants::ring::{COMP_IDX_MASK, TX_COMP, TX_RING};

// SAFETY anchor: see tx.rs.
impl TxRing {
    pub fn reclaim(&mut self) {
        for _ in 0..TX_COMP {
            let c = unsafe { self.comp(self.comp_next) };
            if !unsafe { completed(c, self.comp_gen) } {
                return;
            }
            // The index is the device's only once the generation is seen.
            fence(Ordering::Acquire);
            let idx = unsafe { read_volatile(addr_of!((*c).d0)) } & COMP_IDX_MASK;
            if (idx as usize) < TX_RING {
                self.clean = (idx as u16 + 1) % TX_RING as u16;
            }
            self.comp_next = (self.comp_next + 1) % TX_COMP as u16;
            if self.comp_next == 0 {
                self.comp_gen ^= 1;
            }
        }
    }

    pub fn full(&self) -> bool {
        (self.fill + 1) % TX_RING as u16 == self.clean
    }
}
