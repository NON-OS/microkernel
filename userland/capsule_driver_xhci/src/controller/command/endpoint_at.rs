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

//! One endpoint of a slot, as recovering it needs it: its ring, and the
//! output context the controller keeps its state in.

use crate::contexts::{endpoint_state, EP_STATE_HALTED};
use crate::dma::DmaRegion;
use crate::rings::transfer::TransferRing;

/// Endpoint `dci` of `slot`: the ring it runs, and the output context.
#[derive(Clone, Copy)]
pub struct EndpointAt<'a> {
    pub ring: &'a TransferRing,
    pub output: &'a DmaRegion,
    pub context_size: u8,
    pub slot: u8,
    pub dci: u8,
}

impl EndpointAt<'_> {
    /// Whether the controller says the endpoint is Halted.
    pub fn halted(&self) -> bool {
        endpoint_state(self.output, self.context_size, self.dci) == EP_STATE_HALTED
    }
}
