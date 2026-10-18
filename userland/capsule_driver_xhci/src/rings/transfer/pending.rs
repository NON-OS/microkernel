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
//! How many TRBs on the ring the controller may not have run yet.
use super::state::TransferRing;

impl TransferRing {
    /// TRBs that may wait on the ring for the controller at once: all but
    /// the Link and one, so the producer never writes over a TRB not yet run.
    pub(super) fn max_pending(&self) -> usize {
        self.trbs - 2
    }
    /// A TD on the ring completed, and only the last `left` TRBs, queued
    /// after it, may still wait: an isoch stream keeps its next batch
    /// queued while the one before plays. Buffers kept stay kept until a
    /// full `retire`.
    pub fn retire_but(&self, left: usize) {
        self.pending.set(self.pending.get().min(left));
    }
}
