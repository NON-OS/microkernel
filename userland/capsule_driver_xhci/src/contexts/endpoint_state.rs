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

//! An endpoint's state as the controller keeps it in the output device
//! context (xHCI 1.2 section 6.2.3, EP State in bits 2:0 of dword 0).

use super::slot_copy::read_dw;
use crate::dma::DmaRegion;

const EP_STATE_MASK: u32 = 0x7;
/// A STALL, babble or transaction error halted the endpoint.
pub const EP_STATE_HALTED: u32 = 2;

/// The EP State of endpoint `dci` in the output context `output`.
pub fn endpoint_state(output: &DmaRegion, context_size: u8, dci: u8) -> u32 {
    read_dw(output, context_size, dci as usize, 0) & EP_STATE_MASK
}
