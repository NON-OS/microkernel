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

//! The live state the server loop holds.

use nonos_libc::DmaMapOut;

use crate::codec::Ident;
use crate::constants::ring::{BDL_OFFSET, PERIOD_BYTES, RING_BYTES, RING_OFFSET};
use crate::regs::{Mixer, Port};

pub struct Driver {
    pub dma: DmaMapOut,
    pub bus: Port,
    pub mixer: Mixer,
    pub ident: Ident,
}

impl Driver {
    /// The list's bus address; the grant sits below 4 GiB.
    pub fn bdl_dev(&self) -> u32 {
        (self.dma.device_addr + BDL_OFFSET) as u32
    }

    pub fn ring_dev(&self) -> u32 {
        (self.dma.device_addr + RING_OFFSET) as u32
    }

    /// Period `i` of the ring, as the refill writes it.
    pub fn period(&mut self, i: usize) -> &mut [u8] {
        let at = self.dma.user_va + RING_OFFSET + (i * PERIOD_BYTES) as u64;
        // SAFETY: `i` is below N_PERIODS at every caller, so the period is
        // inside the ring the grant maps, which stays mapped while the
        // driver lives; `&mut self` keeps the slice unique.
        unsafe { core::slice::from_raw_parts_mut(at as *mut u8, PERIOD_BYTES) }
    }

    /// The whole ring zeroed, so a stream starts on silence.
    pub fn silence(&mut self) {
        let at = self.dma.user_va + RING_OFFSET;
        // SAFETY: the ring is RING_BYTES of the grant, as above.
        unsafe { core::slice::from_raw_parts_mut(at as *mut u8, RING_BYTES) }.fill(0);
    }
}
