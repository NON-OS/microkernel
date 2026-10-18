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
use super::state::EventRing;
use crate::constants::TRB_BYTES;
use crate::trb::{read_volatile_at, Trb};
impl EventRing {
    pub fn current_trb(&self) -> Trb {
        let va = self.segment.user_va() + (self.dequeue_index as u64) * (TRB_BYTES as u64);
        // SAFETY: `dequeue_index` is below EVENT_RING_SEGMENT_TRBS (`advance`
        // wraps it), so the TRB lies inside the live segment grant.
        unsafe { read_volatile_at(va) }
    }
}
