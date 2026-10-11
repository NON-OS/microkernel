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

//! Receive queue state: ring 0, which gets every buffer, ring 1, which is
//! handed over empty, the completion ring, and the pool. Buffer `i` belongs
//! to descriptor `i` for good; `fill`/`gen` are the next descriptor to hand
//! over and the generation it goes with.

use super::desc::{Comp, Desc};
use crate::constants::ring::{BUFFER_LEN, INIT_GEN};

pub struct RxRing {
    pub ring_va: u64,
    pub ring1_va: u64,
    pub comp_va: u64,
    pub pool_va: u64,
    pub pool_dev: u64,
    pub fill: u16,
    pub gen: u32,
    pub comp_next: u16,
    pub comp_gen: u32,
}

// SAFETY anchor for every unsafe on RxRing: the three rings are in the
// control grant (RX_RING, RX_RING and RX_COMP descriptors long) and every
// index is held below those.
impl RxRing {
    pub fn new(ring_va: u64, ring1_va: u64, comp_va: u64, pool_va: u64, pool_dev: u64) -> Self {
        let (fill, comp_next, gen, comp_gen) = (0, 0, INIT_GEN, INIT_GEN);
        Self { ring_va, ring1_va, comp_va, pool_va, pool_dev, fill, gen, comp_next, comp_gen }
    }

    /// # Safety
    /// `idx` is below RX_RING, the length of ring 0.
    pub unsafe fn desc(&self, idx: u16) -> *mut Desc {
        (self.ring_va as *mut Desc).add(idx as usize)
    }

    /// # Safety
    /// `idx` is below RX_RING, the length of ring 1.
    pub unsafe fn desc1(&self, idx: u16) -> *mut Desc {
        (self.ring1_va as *mut Desc).add(idx as usize)
    }

    /// # Safety
    /// `idx` is below RX_COMP, the length of the completion ring.
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
