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

//! The ring as the producer sees it: the segment, and how far the driver
//! has handed events back through ERDP.

use super::producer::{reg64, Producer, ERDP_FLAGS, TRB};
use crate::constants::ERDP_LO;

impl Producer {
    pub fn segment_trbs(&self) -> usize {
        self.trbs
    }

    pub fn segment_bus(&self) -> core::ops::Range<u64> {
        self.segment_bus..self.segment_bus + self.trbs as u64 * TRB
    }

    pub fn erdp(&self) -> u64 {
        reg64(&self.regs, ERDP_LO)
    }

    /// The slot ERDP names. A pointer outside the segment fails here: the
    /// controller would have nowhere to take it.
    pub fn dequeue(&self) -> usize {
        let at = self.erdp() & !ERDP_FLAGS;
        assert!(self.segment_bus().contains(&at), "ERDP {at:#x} is outside the segment");
        ((at - self.segment_bus) / TRB) as usize
    }

    /// Events written and not yet handed back.
    pub fn pending(&self) -> usize {
        (self.enqueue + self.trbs - self.dequeue()) % self.trbs
    }

    /// Room for more events before the ring is full.
    pub fn room(&self) -> usize {
        self.trbs - 1 - self.pending()
    }

    pub(super) fn slot_host(&self, at: usize) -> u64 {
        self.segment_host + at as u64 * TRB
    }
}
