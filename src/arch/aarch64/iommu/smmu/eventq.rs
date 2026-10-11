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

//! The event queue: where the unit records faults, among them every
//! transaction a stream's abort STE turned away. The kernel does not drain
//! it yet; once full the unit drops further events, which loses records,
//! never DMA protection.

use crate::memory::iommu::IommuError;

use super::frame;
use super::regs::BASE_RA;

/// One frame of 32-byte events.
const LOG2: u8 = 7;

pub(super) struct Eventq {
    phys: u64,
    log2: u8,
}

impl Eventq {
    pub(super) fn new(max_log2: u8) -> Result<Self, IommuError> {
        Ok(Self { phys: frame::alloc()?, log2: max_log2.min(LOG2) })
    }

    pub(super) fn base(&self) -> u64 {
        BASE_RA | self.phys | u64::from(self.log2)
    }
}
