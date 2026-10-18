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

use crate::constants::TRB_TYPE_TRANSFER_EVENT;
use crate::regs::runtime::erdp_program;
use crate::rings::event::EventRing;
use crate::trb::Trb;
pub const DRAIN_BATCH: usize = 32;
pub struct DrainBatch {
    pub trbs: [Trb; DRAIN_BATCH],
    pub count: usize,
}
impl DrainBatch {
    pub fn new() -> Self {
        Self { trbs: [Trb::zero(); DRAIN_BATCH], count: 0 }
    }
}
/// Take up to `DRAIN_BATCH` events between requests. A transfer event stops
/// the drain: its endpoint's poll takes it. A command completion does not:
/// the drain runs only between commands, so it is an orphan of an abort or
/// of a wait that gave up, and is taken off and counted, where it used to
/// stop the drain until the next command.
pub fn drain_events(intr_base: u64, ring: &mut EventRing) -> DrainBatch {
    let mut batch = DrainBatch::new();
    // Orphans count against the batch too, so a drain stays bounded.
    let mut taken = 0;
    while taken < DRAIN_BATCH && ring.has_event() {
        taken += 1;
        if ring.skip_orphan() {
            continue;
        }
        let trb = ring.current_trb();
        if trb.get_type() == TRB_TYPE_TRANSFER_EVENT {
            break;
        }
        batch.trbs[batch.count] = trb;
        batch.count += 1;
        ring.advance();
    }
    erdp_program(intr_base, ring.current_dequeue_phys(), true, 0);
    batch
}
