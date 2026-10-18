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

//! Handing receive descriptors to the device. The address goes first and
//! the word with the generation bit last: the device may take the slot the
//! moment that bit matches its own.

use core::ptr::{addr_of_mut, write_volatile};
use core::sync::atomic::{fence, Ordering};

use super::desc::{Comp, Desc};
use super::rx::RxRing;
use crate::constants::ring::{BUFFER_LEN, INIT_GEN, RXD_GEN_SHIFT, RX_COMP, RX_RING};

// SAFETY anchor: see rx.rs.
impl RxRing {
    /// Hand over the descriptor at `fill`, whose buffer is free, and return
    /// the new `fill` for RXPROD.
    pub fn post(&mut self) -> u32 {
        let d = unsafe { self.desc(self.fill) };
        unsafe {
            write_volatile(addr_of_mut!((*d).addr), self.buffer_dev(self.fill));
            write_volatile(addr_of_mut!((*d).d3), 0);
        }
        fence(Ordering::Release);
        // Buffer type 0, a head buffer; descriptor type 0.
        let d2 = BUFFER_LEN as u32 | self.gen << RXD_GEN_SHIFT;
        unsafe { write_volatile(addr_of_mut!((*d).d2), d2) };
        self.fill = (self.fill + 1) % RX_RING as u16;
        if self.fill == 0 {
            self.gen ^= 1;
        }
        self.fill as u32
    }

    /// Every ring cleared, then all but one ring 0 descriptor handed over,
    /// as Linux does: the one held back is where the next refill goes.
    /// Returns ring 0's producer index.
    pub fn reset(&mut self) -> u32 {
        for i in 0..RX_RING as u16 {
            unsafe {
                write_volatile(self.desc(i), Desc::default());
                write_volatile(self.desc1(i), Desc::default());
            }
        }
        for i in 0..RX_COMP as u16 {
            unsafe { write_volatile(self.comp(i), Comp::default()) };
        }
        (self.fill, self.comp_next, self.gen, self.comp_gen) = (0, 0, INIT_GEN, INIT_GEN);
        let mut prod = 0;
        for _ in 0..RX_RING - 1 {
            prod = self.post();
        }
        prod
    }
}
