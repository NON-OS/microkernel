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
use super::state::TransferRing;
use crate::constants::TRB_BYTES;
use crate::error::{XhciError, XhciResult};
use crate::trb::builders::link::LinkTrbBuilder;
use crate::trb::{write_volatile_at, Trb};
impl TransferRing {
    /// Queue `trb`. Refused, the ring untouched, while `max_pending` TRBs
    /// are queued that no completion or dequeue move has retired: after a
    /// recovery that failed, writing on would lap the controller.
    pub fn enqueue(&mut self, mut trb: Trb) -> XhciResult<u64> {
        if self.pending.get() >= self.max_pending() {
            return Err(XhciError::TransferRingFull);
        }
        trb.set_cycle(self.cycle != 0);
        let slot_va = self.region.user_va() + (self.enqueue_index as u64) * (TRB_BYTES as u64);
        let slot_phys = self.region.phys() + (self.enqueue_index as u64) * (TRB_BYTES as u64);
        // SAFETY: `enqueue_index` is below the Link's index (`wrap_if_needed`
        // resets it there), so the slot is inside the ring's grant.
        unsafe { write_volatile_at(slot_va, trb) };
        self.enqueue_index += 1;
        self.pending.set(self.pending.get() + 1);
        self.wrap_if_needed();
        Ok(slot_phys)
    }
    /// Refuse, queuing nothing, a TD of `trbs` TRBs the ring has no room
    /// for: queued in part, its data stage named a buffer freed with it.
    pub fn room_for(&self, trbs: usize) -> XhciResult<()> {
        if self.pending.get() + trbs > self.max_pending() {
            return Err(XhciError::TransferRingFull);
        }
        Ok(())
    }
    /// Every TRB queued so far has been consumed: a transfer on the ring
    /// completed, so the controller ran all before it, or the dequeue
    /// pointer was moved to the enqueue pointer. The buffers kept for them
    /// are freed (`keep`).
    pub fn retire(&self) {
        self.pending.set(0);
        drop(self.kept.take());
    }
    fn wrap_if_needed(&mut self) {
        if self.enqueue_index != self.trbs - 1 {
            return;
        }
        let link = LinkTrbBuilder::new()
            .target(self.region.phys())
            .toggle_cycle(true)
            .cycle(self.cycle != 0)
            .build();
        let link_va = self.region.user_va() + ((self.trbs as u64) - 1) * (TRB_BYTES as u64);
        // SAFETY: the last TRB slot of the ring's own grant.
        unsafe { write_volatile_at(link_va, link) };
        self.cycle ^= 1;
        self.enqueue_index = 0;
    }
}
