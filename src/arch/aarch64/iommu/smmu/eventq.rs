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

//! The event queue, where the unit records faults (among them every
//! transaction an abort STE turned away). Drained after every driver call.

use crate::memory::iommu::IommuError;
use crate::sys::serial::Line;

use super::encode::event::{decode, name};
use super::encode::queue::{advance, index, is_empty};
use super::frame;
use super::mmio::Regs;
use super::regs::{BASE_RA, EVENTQ_CONS, EVENTQ_PROD};

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

    /// Report every event waiting, at most one queue's worth, and hand the
    /// slots back. PROD's overflow bit (31) says events were lost while the
    /// queue was full; that is reported too, never ignored.
    pub(super) fn drain(&self, regs: Regs) {
        let Ok(words) = frame::words(self.phys) else { return };
        let mut cons = regs.read32(EVENTQ_CONS) & ((2 << self.log2) - 1);
        for _ in 0..1u32 << self.log2 {
            let prod = regs.read32(EVENTQ_PROD);
            if is_empty(prod, cons, self.log2) {
                if prod & (1 << 31) != 0 {
                    crate::sys::serial::println(b"[SMMU] event queue overflowed; events lost");
                }
                break;
            }
            let at = index(cons, self.log2) as usize * 4;
            let event = decode(&[words[at], words[at + 1], words[at + 2], words[at + 3]]);
            Line::new()
                .str(b"[SMMU] event ")
                .str(name(event.kind))
                .str(b" stream=")
                .hex(u64::from(event.sid))
                .str(b" address=")
                .hex(event.address)
                .end();
            cons = advance(cons, self.log2);
            regs.write32(EVENTQ_CONS, cons | (prod & (1 << 31)));
        }
    }
}
