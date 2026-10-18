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
use crate::dma::DmaRegion;
use crate::rings::transfer::TransferRing;

/// The interrupt-IN endpoints one device may have configured at once. A
/// wireless keyboard and mouse receiver has two, a gaming keyboard three.
pub const MAX_INTERRUPT_ENDPOINTS: usize = 4;

/// One interrupt-IN endpoint: its DCI, its ring, the buffer one report
/// lands in and its length, the endpoint's whole payload per interval, and
/// the TRB armed on it, if any.
pub struct InterruptEndpoint {
    pub dci: u8,
    pub ring: TransferRing,
    pub buf: DmaRegion,
    pub len: u16,
    pub armed: Option<u64>,
}
