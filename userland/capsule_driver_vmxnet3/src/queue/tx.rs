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

//! Transmit queue state: the ring the driver fills, the completion ring
//! the device fills, and the pool. `fill`/`gen` are where the next frame
//! goes and the generation it is posted with; `clean` is the oldest slot not
//! yet seen completed. One slot always stays empty, so `fill == clean`
//! means empty and never full.

use super::desc::{Comp, Desc};
use crate::constants::ring::{BUFFER_LEN, INIT_GEN};

pub struct TxRing {
    pub ring_va: u64,
    pub comp_va: u64,
    pub pool_va: u64,
    pub pool_dev: u64,
    pub fill: u16,
    pub gen: u32,
    pub clean: u16,
    pub comp_next: u16,
    pub comp_gen: u32,
}

// SAFETY anchor for every unsafe on TxRing: `ring_va` and `comp_va` are in
// the control grant, TX_RING and TX_COMP descriptors long, and every index
// is held below those.
impl TxRing {
    pub fn new(ring_va: u64, comp_va: u64, pool_va: u64, pool_dev: u64) -> Self {
        let (fill, clean, comp_next) = (0, 0, 0);
        let (gen, comp_gen) = (INIT_GEN, INIT_GEN);
        Self { ring_va, comp_va, pool_va, pool_dev, fill, gen, clean, comp_next, comp_gen }
    }

    /// # Safety
    /// `idx` is below TX_RING, the length of the transmit ring.
    pub unsafe fn desc(&self, idx: u16) -> *mut Desc {
        (self.ring_va as *mut Desc).add(idx as usize)
    }

    /// # Safety
    /// `idx` is below TX_COMP, the length of the completion ring.
    pub unsafe fn comp(&self, idx: u16) -> *mut Comp {
        (self.comp_va as *mut Comp).add(idx as usize)
    }

    pub fn buffer_dev(&self, idx: u16) -> u64 {
        self.pool_dev + idx as u64 * BUFFER_LEN as u64
    }

    pub fn buffer_va(&self, idx: u16) -> u64 {
        self.pool_va + idx as u64 * BUFFER_LEN as u64
    }
}
