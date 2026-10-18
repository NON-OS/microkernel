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
use super::poll::IntrPoll;
use super::report::completed;
use crate::constants::{EVENT_RING_SEGMENT_TRBS, TRB_TYPE_TRANSFER_EVENT};
use crate::error::XhciResult;
use crate::regs::runtime::erdp_program;
use crate::rings::event::{EventRing, IssuedTransfer};
use crate::slots::InterruptEndpoint;
use crate::trb::Trb;
pub fn scan(
    intr_base: u64,
    evt_ring: &mut EventRing,
    slot: u8,
    ep: &mut InterruptEndpoint,
    length: u16,
) -> XhciResult<IntrPoll> {
    let issued = IssuedTransfer { phys: ep.armed.unwrap_or(0), slot, dci: ep.dci };
    if let Some(event) = evt_ring.take_parked(issued) {
        return finish(ep, &event, length);
    }
    // At most one lap: the ring holds no more events than that.
    for _ in 0..EVENT_RING_SEGMENT_TRBS {
        if !evt_ring.has_event() {
            break;
        }
        let event = evt_ring.current_trb();
        evt_ring.advance();
        erdp_program(intr_base, evt_ring.current_dequeue_phys(), true, 0);
        if issued.completed_by(&event) {
            evt_ring.settle(issued);
            return finish(ep, &event, length);
        }
        /*
         * Another armed endpoint's completion is kept for its own poll, so
         * it neither blocks this one nor is lost (`park`). Anything else is
         * nobody's: no command is in flight inside a poll, so a command
         * completion here is a late answer to one given up, and left at the
         * head it would hold every report behind it.
         */
        if event.get_type() == TRB_TYPE_TRANSFER_EVENT {
            evt_ring.park(event);
        }
    }
    Ok(IntrPoll::Pending)
}
/// The armed TRB's event: nothing is armed any more, and a report that
/// came leaves the ring with nothing the controller has not run. Of the
/// bytes it moved, at most `length` are the client's.
fn finish(ep: &mut InterruptEndpoint, event: &Trb, length: u16) -> XhciResult<IntrPoll> {
    ep.armed = None;
    let got = completed(event, ep.len)?;
    ep.ring.retire();
    Ok(match got {
        IntrPoll::Complete(n) => IntrPoll::Complete(n.min(length)),
        pending => pending,
    })
}
