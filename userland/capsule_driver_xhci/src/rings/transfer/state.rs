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
use core::cell::Cell;

use crate::constants::{TRANSFER_RING_TRBS, TRB_BYTES};
use crate::dma::{DmaPool, DmaRegion};
use crate::error::XhciResult;
use crate::trb::builders::link::LinkTrbBuilder;
use crate::trb::write_volatile_at;
use alloc::vec::Vec;
pub struct TransferRing {
    pub(super) region: DmaRegion,
    /// TRB slots in `region`, the Link in the last of them.
    pub(super) trbs: usize,
    pub(super) cycle: u8,
    pub(super) enqueue_index: usize,
    /// TRBs queued that the controller is not known to have consumed
    /// (`retire`). A cell, so the paths that hold the ring shared, endpoint
    /// recovery among them, can retire it.
    pub(super) pending: Cell<usize>,
    /// Request buffers that TRBs still on the ring point at (`keep`).
    pub(crate) kept: Cell<Vec<DmaRegion>>,
}
impl TransferRing {
    pub fn new(pool: &DmaPool) -> XhciResult<Self> {
        Self::with_trbs(pool, TRANSFER_RING_TRBS)
    }
    /// A ring of `trbs` TRB slots, more than two, the Link in the last.
    pub fn with_trbs(pool: &DmaPool, trbs: usize) -> XhciResult<Self> {
        let bytes = (trbs as u64) * (TRB_BYTES as u64);
        let region = pool.alloc(bytes)?;
        region.zero();
        let link_va = region.user_va() + ((trbs as u64) - 1) * (TRB_BYTES as u64);
        let link =
            LinkTrbBuilder::new().target(region.phys()).toggle_cycle(true).cycle(true).build();
        // SAFETY: the last TRB slot of the ring's own grant, just mapped.
        unsafe { write_volatile_at(link_va, link) };
        Ok(Self {
            region,
            trbs,
            cycle: 1,
            enqueue_index: 0,
            pending: Cell::new(0),
            kept: Cell::default(),
        })
    }
}
