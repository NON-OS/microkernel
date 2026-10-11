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

//! The receive DMA region as the poll path reads it.

use crate::fw::dma::{DmaMem, Grant};

/// The receive DMA region as the poll path needs to see it: a CPU-readable view
/// of the buffer bytes plus the device address the card re-arms against. The
/// bring-up backs this with the mapped grant; a proof backs it with plain memory.
pub trait RxBuffers {
    /// The buffer bytes, for reading a completed frame out of the ring.
    fn bytes(&self) -> &[u8];
    /// The bus address of byte zero, for re-arming descriptors.
    fn device_addr(&self) -> u64;
}

/// A live broker grant is the receive buffer region on real hardware: its device
/// address re-arms descriptors and its mapped bytes carry the frames the card
/// wrote. The proofs back this with plain memory instead.
impl RxBuffers for Grant {
    fn bytes(&self) -> &[u8] {
        self.as_slice()
    }
    fn device_addr(&self) -> u64 {
        DmaMem::device_addr(self)
    }
}
