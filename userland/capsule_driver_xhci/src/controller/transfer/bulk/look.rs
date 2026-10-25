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
//! Looking at the event ring for a polled TRB's completion without waiting:
//! the events already written, one lap at most. Another armed endpoint's
//! completion is kept for its own poll (`park`). Anything else is nobody's,
//! a command completion among them: no command is in flight inside a poll,
//! and one left at the head would hold the poll's own completion behind it.
use crate::constants::{EVENT_RING_SEGMENT_TRBS, TRB_TYPE_TRANSFER_EVENT};
use crate::regs::runtime::erdp_program;
use crate::rings::event::{EventRing, IssuedTransfer};
use crate::trb::Trb;

/// The event of `issued`, its place given back; `None` while none came.
pub(super) fn look(
    intr_base: u64,
    evt_ring: &mut EventRing,
    issued: IssuedTransfer,
) -> Option<Trb> {
    if let Some(event) = evt_ring.take_parked(issued) {
        return Some(event);
    }
    for _ in 0..EVENT_RING_SEGMENT_TRBS {
        if !evt_ring.has_event() {
            break;
        }
        let event = evt_ring.current_trb();
        evt_ring.advance();
        erdp_program(intr_base, evt_ring.current_dequeue_phys(), true, 0);
        if issued.completed_by(&event) {
            evt_ring.settle(issued);
            return Some(event);
        }
        if event.get_type() == TRB_TYPE_TRANSFER_EVENT {
            evt_ring.park(event);
        }
    }
    None
}
