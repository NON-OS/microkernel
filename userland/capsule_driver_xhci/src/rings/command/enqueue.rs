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
use super::state::CommandRing;
use crate::constants::{COMMAND_RING_TRBS, TRB_BYTES};
use crate::error::{XhciError, XhciResult};
use crate::trb::builders::link::LinkTrbBuilder;
use crate::trb::{write_volatile_at, Trb};
/// Commands that may wait on the ring at once: all but the Link and one.
const MAX_PENDING: usize = COMMAND_RING_TRBS - 2;
impl CommandRing {
    /// Queue `trb`. Refused, the ring untouched, while `MAX_PENDING`
    /// commands are queued that no completion has retired: commands given
    /// up unanswered stay queued as No Ops, and writing on would lap the
    /// controller once it runs.
    pub fn enqueue(&mut self, mut trb: Trb) -> XhciResult<u64> {
        if self.pending >= MAX_PENDING {
            return Err(XhciError::CommandRingFull);
        }
        trb.set_cycle(self.cycle != 0);
        let slot_va = self.region.user_va() + (self.enqueue_index as u64) * (TRB_BYTES as u64);
        let slot_phys = self.region.phys() + (self.enqueue_index as u64) * (TRB_BYTES as u64);
        // SAFETY: `enqueue_index` is below the Link's index (reset to 0 when
        // it reaches it), so the slot is inside the ring's grant.
        unsafe { write_volatile_at(slot_va, trb) };
        self.enqueue_index += 1;
        self.pending += 1;
        if self.enqueue_index == COMMAND_RING_TRBS - 1 {
            let link = LinkTrbBuilder::new()
                .target(self.region.phys())
                .toggle_cycle(true)
                .cycle(self.cycle != 0)
                .build();
            let link_va =
                self.region.user_va() + ((COMMAND_RING_TRBS as u64) - 1) * (TRB_BYTES as u64);
            // SAFETY: the last TRB slot of the ring's own grant.
            unsafe { write_volatile_at(link_va, link) };
            self.cycle ^= 1;
            self.enqueue_index = 0;
        }
        Ok(slot_phys)
    }
    /// Every command queued so far has been consumed: one completed, so
    /// the controller ran all before it.
    pub fn retire(&mut self) {
        self.pending = 0;
    }
}
