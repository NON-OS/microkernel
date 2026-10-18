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

//! What one isoch OUT endpoint holds: its ring, two halves of DMA memory a
//! batch each, and the last TD of the batch still in flight.

use super::endpoint::IsochEndpoint;
use crate::dma::{DmaPool, DmaRegion};
use crate::error::XhciResult;
use crate::protocol::{ISOCH_MAX, ISOCH_PACKETS_MAX};
use crate::rings::event::IssuedTransfer;
use crate::rings::transfer::TransferRing;

/// Room for two whole batches the controller has not run, with the Link
/// and the slot the producer keeps free: a default ring holds 62.
const ISOCH_RING_TRBS: usize = 2 * ISOCH_PACKETS_MAX + 2;

pub struct IsochPipe {
    pub ring: TransferRing,
    pub buf: DmaRegion,
    pub ep: IsochEndpoint,
    pub superspeed: bool,
    /// The half the next batch is written to.
    pub(super) half: usize,
    /// The last TD of the batch queued before, whose completion the next
    /// batch waits for; a place is kept for its event (`expect`). Two
    /// batches at most are on the ring, which has room for both.
    pub(super) pending: Option<IssuedTransfer>,
}

impl IsochPipe {
    pub fn memory(pool: &DmaPool) -> XhciResult<(TransferRing, DmaRegion)> {
        let buf = pool.alloc(2 * ISOCH_MAX as u64)?;
        buf.zero();
        Ok((TransferRing::with_trbs(pool, ISOCH_RING_TRBS)?, buf))
    }

    pub fn with(ring: TransferRing, buf: DmaRegion, ep: IsochEndpoint, superspeed: bool) -> Self {
        Self { ring, buf, ep, superspeed, half: 0, pending: None }
    }

    /// Forget the batch in flight: the endpoint was recovered past it.
    pub fn forget(&mut self) {
        self.pending = None;
    }
}
