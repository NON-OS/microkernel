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

//! The two-level stream table. Level 1 is one frame of 512 descriptors;
//! each level 2 table is one frame of 64 STEs, allocated when a stream in
//! its range is first given a translation.

use crate::memory::iommu::IommuError;

use super::frame;
use super::regs::{BASE_RA, STRTAB_FMT_2LVL};

/// STEs per level 2 table: 64 of 64 bytes fill one frame.
pub(super) const SPLIT: u8 = 6;
/// Level 1 descriptors: 512 of 8 bytes fill one frame.
const L1_LOG2: u8 = 9;

pub(super) struct StreamTable {
    pub(super) l1: u64,
    pub(super) l2: [u64; 1 << L1_LOG2],
    log2size: u8,
}

impl StreamTable {
    /// Streams past `1 << (SPLIT + L1_LOG2)` are left out of range, so the
    /// unit aborts them; a unit decoding fewer bits than one level 2 table
    /// covers still gets a whole one.
    pub(super) fn new(sid_bits: u8) -> Result<Self, IommuError> {
        let log2size = sid_bits.clamp(SPLIT, SPLIT + L1_LOG2);
        Ok(Self { l1: frame::alloc()?, l2: [0; 1 << L1_LOG2], log2size })
    }

    pub(super) fn base(&self) -> u64 {
        BASE_RA | self.l1
    }

    pub(super) fn config(&self) -> u32 {
        STRTAB_FMT_2LVL | u32::from(SPLIT) << 6 | u32::from(self.log2size)
    }

    pub(super) fn covers(&self, sid: u32) -> bool {
        u64::from(sid) < 1 << self.log2size
    }
}
