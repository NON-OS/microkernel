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
//! The bulk IN and OUT endpoints of one device, for a mass-storage class
//! driver above this one: a transfer ring each and one page of DMA memory
//! that every bulk transfer on the slot moves through.
use crate::dma::{DmaPool, DmaRegion};
use crate::error::XhciResult;
use crate::protocol::BULK_MAX;
use crate::rings::transfer::TransferRing;
pub struct BulkPipes {
    pub in_ring: TransferRing,
    pub out_ring: TransferRing,
    pub buf: DmaRegion,
    pub dci_in: u8,
    pub dci_out: u8,
}
impl BulkPipes {
    pub fn new(pool: &DmaPool, dci_in: u8, dci_out: u8) -> XhciResult<Self> {
        let buf = pool.alloc(BULK_MAX as u64)?;
        buf.zero();
        Ok(Self {
            in_ring: TransferRing::new(pool)?,
            out_ring: TransferRing::new(pool)?,
            buf,
            dci_in,
            dci_out,
        })
    }
}
