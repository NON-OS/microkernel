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
use crate::dma::DmaRegion;

impl TransferRing {
    /// Keep `buf`, a request buffer a TRB on this ring points at that may
    /// still run, until `retire`: a later transfer completing, or the
    /// dequeue moved, proves the controller passed it. Until then a freed
    /// frame the kernel handed on could be written. Kept on the slot until
    /// Disable Slot, these grew by one for every request whose recovery the
    /// controller refused. Each buffered request queues at least two TRBs
    /// that only `retire` gives back, so no more than `MAX_PENDING` / 2
    /// are kept before the ring refuses new ones.
    pub fn keep(&self, buf: DmaRegion) {
        let mut kept = self.kept.take();
        kept.push(buf);
        self.kept.set(kept);
    }

    /// The ring's own memory and every buffer kept with it, for a slot to
    /// keep whole when a failed controller may have been given the ring.
    pub fn into_regions(self) -> impl Iterator<Item = DmaRegion> {
        let TransferRing { region, kept, .. } = self;
        core::iter::once(region).chain(kept.into_inner())
    }
}
