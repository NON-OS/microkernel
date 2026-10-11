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
//! One bulk transfer through the slot's DMA page: a single Normal TRB, the
//! One look at the event ring for a transfer's completion, without waiting.
//! Another endpoint's completion is parked for its own waiter or poll, and a
//! command's completion is left in place for the command wait to take.
use crate::constants::TRB_TYPE_TRANSFER_EVENT;
use crate::regs::runtime::erdp_program;
use crate::rings::event::{EventRing, IssuedTransfer};
use crate::trb::Trb;

pub(super) fn look(
    intr_base: u64,
    evt_ring: &mut EventRing,
    issued: IssuedTransfer,
) -> Option<Trb> {
    if let Some(event) = evt_ring.take_parked(issued) {
        return Some(event);
    }
    if !evt_ring.has_event() {
        return None;
    }
    let event = evt_ring.current_trb();
    let ours = issued.completed_by(&event);
    if !ours && event.get_type() != TRB_TYPE_TRANSFER_EVENT {
        return None;
    }
    evt_ring.advance();
    erdp_program(intr_base, evt_ring.current_dequeue_phys(), true, 0);
    if !ours {
        evt_ring.park(event);
        return None;
    }
    Some(event)
}
